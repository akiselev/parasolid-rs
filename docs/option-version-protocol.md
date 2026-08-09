# The `o_t_version` protocol

How to establish the real layout of a Parasolid option struct, and why the
documented one is usually the wrong answer.

This is Stage 0 item 4 of [`CADABRA3-PRIORITIES.md`](../CADABRA3-PRIORITIES.md).
It exists because the same defect keeps recurring in different subsystems, and
each time it has been rediscovered from scratch.

## The mechanism

Almost every non-trivial PK entry point takes `const PK_<thing>_o_t *options`.
The first four bytes are `o_t_version`. The kernel does not read your struct
directly: it runs a **migration routine** that copies your (older, smaller)
*user* struct field by field into the *internal* struct the current build
actually uses, defaulting everything you did not supply.

That gives two different structs with the same name:

| | user struct | internal struct |
|---|---|---|
| who writes it | the caller | the kernel's migration routine |
| size | small, version-dependent | large, build-dependent |
| what the docs describe | rarely | usually |
| what `pk-option-structs.md` computes | sometimes | often |

**The documented layout is normally the internal one.** Modelling it and
stamping `o_t_version = 1` is the single most common way to produce a call that
either fails outright or — much worse — succeeds while reading your fields at
the wrong offsets.

Worked evidence:

- `PK_BODY_boolean_2_o_t` — the binding modelled the 192-byte v19 internal
  layout and set version 1. The kernel returned `o_t_version_incorrect` (5043)
  for that version and accepts only **2..=19**. The real v2 user struct is
  **32 bytes**: `{o_t_version, function@4, configuration@8 (ptr),
  default_tol@16 (f64), 3×u8 flags@24..26, fence@28}`. A NULL `configuration`
  is auto-filled, so no nested sub-struct is needed at all.
- `PK_TOPOL_eval_mass_props_o_t` — accepts **1..=7**. The v1 user struct is
  `{o_t_version, mass@4, periphery@8, bound@12, single@16 (1 byte)}`; the
  internal v7 form adds `use_facets`, `facet_tol`, densities, `transfs` and
  scale fields that a v1 caller never supplies.

Both numbers are reproduced on demand by
`crates/parasolid-test/src/bin/option_version_probe.rs`.

## The two error codes

The kernel distinguishes them, and the distinction is the most useful signal in
the whole procedure:

| code | token | meaning |
|---:|---|---|
| 5022 | `PK_ERROR_o_t_version_unknown` | this build has never heard of that version — you are outside the range entirely |
| 5043 | `PK_ERROR_o_t_version_incorrect` | the version exists, but this entry point will not take it |

A sweep that returns *unknown* everywhere means the version field is not where
you think it is. A sweep with a clean accepted band bounded by *unknown* on both
sides means the field is correct and you have found the window.

Both values are probed, not documented — see `parasolid_sys::error_codes`.

## The procedure

### 1. Sweep the accepted versions — before decompiling anything

Add an arm to `option_version_probe.rs`. It takes a closure that stamps a
version and returns the raw code, and reports the accepted set:

```
=== PK_TOPOL_eval_mass_props_o_t
  swept 0..=24: accepted 7 (1..=7), unknown 18, incorrect 0

=== PK_BODY_boolean_2_o_t
  swept 0..=24: accepted 18 (2..=19), unknown 6, incorrect 1
```

Keep the operands valid but cheap, and **never reuse an operand across
attempts** — an accepted version runs the real operation, and a successful
boolean consumes its target. Reusing it turns the probe into a crash rather
than a result.

The lowest accepted version is the one to target. It has the smallest user
struct and the most defaulting done for you.

### 2. Decompile the migration routine

Find the routine that switches on `o_t_version` — for mass props it is
`FUN_180441cd0`, for booleans `FUN_18049b860`. It reads the caller's struct at
the offsets you need and writes the internal one. Its `switch (version)` arms
tell you exactly which fields each version added.

With the ghidra-cli bridge in `~/projects/parasolid-re`:

```bash
export GHIDRA_PROJECT_DIR=$PWD/work/ghidra-projects
ghidra start --project parasolid-c900fa3f430f --program pskernel.dll
ghidra decompile <FUN_or_symbol> --project parasolid-c900fa3f430f --program pskernel.dll
```

### 3. Prefer the journal helper when one exists

If `PKU_journal_<TYPE>_o` or `_r` exists, decompile **that** instead. These
helpers walk the struct field by field to write the journal file, and they pass
the *field name as a string literal* to each writer:

```c
FUN_180a753e0(*param_1,"r_t_version");
PKU_journal_sym(0x5b,"regions");
PKU_journal_LOGICAL(*(...),"senses");
```

That yields names, order, element types and array/pointer-ness in one pass. It
is how `PK_LATTICE_ask_regions_r_t` was fully recovered — including that
`senses` is a byte array while `regions` and `frames` are 4-byte tag arrays —
for a function with no documented prototype anywhere.

Check `catalog/dylib-symbols.tsv` in `parasolid-re` for the available helpers;
there are 479 of them.

### 4. Confirm each field at runtime

Static recovery is a hypothesis. Set one field to a value whose effect is
observable and assert the effect, field by field. A struct that "works" because
every field happens to be zero has proved nothing.

### 5. Record it

Note the accepted version range, the user-struct layout, the migration routine's
address, and which fields are runtime-confirmed versus inferred. Layouts go in
`docs/pskernel-solidworks.md`; grades reconcile with
`parasolid-re/catalog/pk-signatures.tsv`.

## Where this still needs applying

Ordered by the stage that needs it:

- `PK_BODY_imprint_o_t` and `PK_BODY_imprint_plane_o_t` (Stage 12) — the
  24-byte plane layout is computed in `parasolid-re/catalog/pk-option-structs.md`
  but is still an opaque stub here.
- `PK_TOPOL_facet_2_o_t` and its nested choice/mesh structs (Stage 15).
- The transmit/receive option surface (Stage 14).
- `PK_TOPOL_eval_mass_props_o_t` versions 2..=7, if a case ever needs
  `facet_tol`, densities or `transfs`.

## The rule

Never stamp `o_t_version = 1` because the struct definition starts there. Sweep
first. If the accepted band does not include 1, the layout you are holding is
the internal one and it is wrong for a caller.
