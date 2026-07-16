# Correcting `parasolid-sys` from the reverse-engineered PK ABI

This repo's FFI (`crates/parasolid-sys`) was hand-written and is **known to contain
wrong values** — most importantly its enum constants are sequential *placeholders*
(`PK_mass_m_of_i_c = 3`) when the real Parasolid tokens are large (`= 14004`). A
separate project has since reverse-engineered the **authoritative** PK ABI for this
exact binary. This document tells a future agent how to correct `parasolid-sys` from
that data with **minimal `ghidra-cli` use** (all the answers are pre-extracted into
plain TSV/`.h` files — you almost never need to open Ghidra).

## Where the authoritative data lives

Repo: **`/home/dev/projects/parasolid-re`** (binary `pskernel.dll`,
Parasolid **V37.01.243**, V35-era tokens — the SAME DLL as `parasolid-rs/lib/pskernel.dll`,
SHA-256 `c900fa3f430fe67c2adb15a50f9604d9812ed81a02c5006b66689dac28263073`).

Read `parasolid-re/catalog/README.md` first. The files you need (no Ghidra required):

| File | Use it to fix |
|---|---|
| `catalog/pk-enum-values.tsv` + `catalog/pk-enums.h` | **Enum constant VALUES** — 654 enums, 3,558 tokens, `NAME = value`. Authoritative. |
| `catalog/pk-clean-prototypes.tsv` | **Function signatures** — 1,061 clean C prototypes (`PK_ERROR_code_t PK_X(TYPE arg, …)`). |
| `catalog/pk-signatures.tsv` | Per-function: prototype source, option-struct name, binary-confirmed. |
| `catalog/pk-option-structs.tsv` + `.md` | **Option-struct layouts** — all 365 `_o_t`, field name:type@offset, x86-64. |
| `catalog/pk-base-types.md` | Base typedefs (`PK_BOX_t`=double[6], `PK_VECTOR_t`, `PK_LOGICAL_t`, tag types). |
| `catalog/pk-reference.tsv` | Raw vendor prototypes + `_o_t` names + descriptions (1,082). |
| `catalog/next-targets.tsv` | Which functions matter most (RE priority) — see `catalog/next-targets.md`. |

Provenance (why to trust it): enum values come from Siemens' own **"PK Token Codes
(Numeric)"** V35 appendix (`parasolid-re/artifacts/enum-values/pk-tokens-v35-qsolid.tsv`)
**cross-validated** against `ThraceShah/PKToy`'s C# binding
(`/home/dev/projects/solidworks-notes/headers/pktoy-binding/parasolid.g.cs`) AND against
16 value bands recovered directly from the binary — three independent sources, **zero
conflicts**. Signatures are the vendor reference, binary-confirmed 18/18 on a sample.

## Correction procedure (data-driven, minimal Ghidra)

Work module-by-module in `crates/parasolid-sys/src/*.rs`. For each:

1. **Enum constants (highest priority — these are the known-wrong bit).**
   For every `pub const PK_..._c = N;`, look the token up in `pk-enum-values.tsv`
   (or `pk-enums.h`, which is already `#define NAME value`) and replace `N` with the
   authoritative value. Best done as codegen: parse `pk-enums.h`, regenerate the
   `parasolid-sys` const blocks. Cross-check: `PK_mass_m_of_i_c` must be `14004`,
   `PK_LOOP_type_outer_c` = `5412`, `PK_EDGE_convexity_smooth_flat_c` = `23590`.
   `PKToy`'s `parasolid.g.cs` is a ready-made second source you can diff against
   (but it is unlicensed — cross-reference only, do not copy verbatim).

2. **Function signatures.** Compare each `extern "C"` decl against
   `pk-clean-prototypes.tsv` (match by function name). Fix arg count, types, and
   order. Map C→Rust with the base types in `pk-base-types.md`
   (tags = `i32`, `PK_LOGICAL_t` = `i32`, `PK_VECTOR_t` = `[f64;3]`,
   `PK_BOX_t` = `[f64;6]`, pointers as `*mut`/`*const`).

3. **Option structs.** Compare each `PK_*_o_t` in `src/*.rs` against
   `pk-option-structs.tsv`/`.md` (field order, types, byte offsets). One known v37
   drift: in `PK_TOPOL_eval_mass_props_o_t` the `single`/`use_facets` fields are
   packed as `u8` at offsets 16/17 (the docs imply 4-byte `PK_LOGICAL_t`) — the
   binary/handoff layout wins. See `parasolid-re/docs/HANDOFF-validated-abi.md`.

4. **Coverage gaps.** `docs/ffi-missing.md` lists the 221 unbound exports; the
   authoritative export list is `parasolid-re/catalog/pk-functions.tsv` (all 1,204).

## Validate (dynamic — the real ground truth)

Do NOT trust static data blindly for the last mile: run the existing harness against
the real kernel. `cargo run -p parasolid-test --target x86_64-pc-windows-gnu` (Wine +
`lib/pskernel.dll`; see `docs/pskernel-solidworks.md`). A corrected enum/signature is
confirmed when a closed-form primitive test (block/sphere/cyl/cone/torus mass/box)
returns the documented value. Fold confirmed fixes back as `dynamic-observed`.

## When you DO need Ghidra (rare)

Only to disambiguate one value/signature the tables can't settle. Minimal invocation:

```bash
export GHIDRA_PROJECT_DIR=/home/dev/projects/parasolid-re/work/ghidra-projects
ghidra decompile PK_TOPOL_find_box --project parasolid-c900fa3f430f --program pskernel.dll --json
```

The project is already **fully typed** (PK signatures, 643 enum types, 365 option
structs applied), so decompiled PK functions read cleanly. Do not run bulk Ghidra
scans — the TSVs already contain the bulk answers. See
`parasolid-re/memory`/`docs/RE_PLAN.md` for the full method and
`~/.claude/.../ghidra-cli-quirks` (`--limit 0` = empty; use `--limit 1000000`; don't
`--pretty` huge dumps; shared daemon — don't `ghidra stop` if another agent is attached).

## If you extend the bindings

Prioritise by `catalog/next-targets.tsv` (impact = high/medium/low). The high-impact
set is Parasolid's numeric core — surface/surface intersection (`ISS`/`MAR`/`REL`),
curve intersection (`ICC`/`QCS`/`SOL`), and distance (`DIS`). Those are internal
(non-PK) routines; binding them is a research task, not a mechanical one.
