//! Raw FFI bindings for the Parasolid PK_* C API.
//!
//! This crate provides Rust type definitions, enum constants, and `extern "C"` function
//! declarations matching the Parasolid kernel API exported by `pskernel.dll`.
//!
//! # Usage
//!
//! Without the `runtime` feature, this crate is purely a type/signature crate with no
//! dependencies. Enable the `runtime` feature to get a `PsKernel` loader struct that
//! dynamically loads `pskernel.dll` via `libloading`.
//!
//! # Safety
//!
//! All FFI functions are inherently unsafe. The caller must ensure:
//! - Parasolid session is initialized (`PK_SESSION_start`)
//! - Tags are valid for the current session
//! - Pointer arguments point to valid, correctly-sized buffers

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// Core primitive types used throughout the API
mod types;
pub use types::*;

// Entity class hierarchy
mod class;
pub use class::*;

// Session and initialization
mod session;
pub use session::*;

// Frustrum callbacks
mod frustrum;
pub use frustrum::*;

// Error handling
mod error;
pub use error::*;

// Model structure and topology
mod topology;
pub use topology::*;

// Geometry types and functions
mod geometry;
pub use geometry::*;

// B-curves and B-surfaces
mod bgeom;
pub use bgeom::*;

// Transformations
mod transform;
pub use transform::*;

// Assemblies and instances
mod assembly;
pub use assembly::*;

// File I/O (transmit/receive)
mod fileio;
pub use fileio::*;

// Enquiry and interrogation functions
mod enquiry;
pub use enquiry::*;

// Mass properties
mod mass;
pub use mass::*;

// Distance and clash detection
mod distance;
pub use distance::*;

// Checking and validation
mod checking;
pub use checking::*;

// Euler operations
mod euler;
pub use euler::*;

// Boolean operations
mod boolean;
pub use boolean::*;

// Blending (edge, face-face, three-face)
mod blend;
pub use blend::*;

// Offsetting, hollowing, thickening
mod offset;
pub use offset::*;

// Sweeping, extrusion, lofting
mod sweep;
pub use sweep::*;

// Sheet and wire modeling
mod sheet;
pub use sheet::*;

// Model editing (face change, delete, simplify, etc.)
mod editing;
pub use editing::*;

// Faceting and rendering
mod facet;
pub use facet::*;

// Attributes and application data
mod attrib;
pub use attrib::*;

// Partitions and rollback
mod partition;
pub use partition::*;

// Debug and version control
mod debug;
pub use debug::*;

// Frame operations
mod frame;
pub use frame::*;

// Lattice geometry
mod lattice;
pub use lattice::*;
