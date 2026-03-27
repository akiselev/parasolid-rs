#![allow(non_upper_case_globals)]

//! Safe Rust wrapper for the Parasolid PK_* C API.
//!
//! This crate provides a safe, ergonomic interface over the raw FFI bindings in
//! `parasolid-sys`. It enforces session lifecycle, error handling, and memory
//! management invariants through Rust's type system.
//!
//! # Quick Start
//!
//! ```no_run
//! use parasolid::{Session, SessionConfig};
//!
//! let session = Session::start(SessionConfig::new())?;
//! let version = session.kernel_version()?;
//! println!("Parasolid {}.{}.{}", version.0, version.1, version.2);
//! // Session is stopped automatically when dropped
//! # Ok::<(), parasolid::PsError>(())
//! ```

#[macro_use]
mod error;
mod memory;
mod frustrum;
mod session;
mod partition;
mod entity;

pub use error::{BadArg, ErrorDetails, PsError, PsResult, Severity};
pub use memory::PkArray;
pub use frustrum::FrustrumConfig;
pub use session::{Behaviour, Session, SessionConfig};
pub use partition::Partition;
pub use entity::{Entity, PkClass};
