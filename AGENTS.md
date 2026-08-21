# Agent instructions

Read `CLAUDE.md`, `README.md`, `TODO.md`, and the relevant documentation before
substantial work. Parasolid-rs provides Rust FFI bindings, safe wrappers, and
the licensed-kernel integration harness used as CADabra's external oracle.

Commit coherent, verified changes directly to `master` as the binding and
oracle surface expands. Do not create feature branches or pull requests.
Recheck the worktree before every commit so another agent's changes are not
staged accidentally.

Never commit `pskernel.dll`, `libpskernel.a`, Parasolid headers, generated import
libraries, credentials, or other proprietary Siemens material. Licensed files
remain under the ignored local `lib/` directory.

Treat unaudited FFI declarations, option layouts, and token values as suspect.
Every binding relied on by CADabra needs a signature audit, an assertion-based
runtime test against the installed kernel, probes for undocumented values, and
documented residual risk. A call merely returning success is not sufficient
evidence.

Keep ordinary source checks independent of licensed runtime availability. When
the kernel is available, run the Windows GNU target integration harness under
Wine and report explicit pass, failure, and skip counts. Unavailable licensed
infrastructure is a reported skip, never an inferred pass.

Before handoff run the narrow relevant checks followed by formatting, workspace
checks, clippy with warnings denied where supported, tests, documentation checks,
and `git diff --check`. Report the exact commit, dirty state, checks, licensed
oracle availability, and whether the commit was pushed.
