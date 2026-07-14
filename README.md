# parasolid-rs

Rust FFI bindings (`parasolid-sys`) and a safe wrapper (`parasolid`) for the
Siemens **Parasolid** geometric modeling kernel (`pskernel`), plus an
integration test harness (`parasolid-test`).

## ⚠️ Proprietary binary — not included

This repository does **not** ship the Parasolid kernel. `pskernel.dll`,
`libpskernel.a`, and the Parasolid headers are proprietary Siemens
intellectual property and must never be committed here. They are covered by
your SOLIDWORKS / Parasolid license.

To build and run, obtain the kernel from a source you are licensed to use and
place it under `lib/` (which is git-ignored):

```
lib/
  pskernel.dll      # required at runtime (Windows / Wine)
  libpskernel.a     # import library, for linking with the *-windows-gnu target
```

On a machine with SOLIDWORKS installed, `pskernel.dll` ships in the SOLIDWORKS
program directory (e.g. `C:\Program Files\SOLIDWORKS Corp\SOLIDWORKS\`). Copy it
into `lib/`. Verified working against Parasolid **V37.01.243** (SOLIDWORKS 2025).

## Build & test

See `docs/pskernel-solidworks.md` for the full cross-compile + Wine workflow.
In short:

```bash
cargo build --workspace --target x86_64-pc-windows-gnu
# ensure lib/pskernel.dll is next to the test exe or on PATH/WINEPATH
cargo run -p parasolid-test --target x86_64-pc-windows-gnu
```

## Validation status

The FFI surface is being validated incrementally against the real kernel — see
`TODO.md` for the roadmap and current coverage. The end goal is to use Parasolid
as a **golden oracle** for the CADabra geometric kernel.
