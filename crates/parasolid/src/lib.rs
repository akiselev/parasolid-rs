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
mod geom;
mod body;
mod mass;
mod range;
mod topology;
mod face;
mod edge;
mod vertex;
mod surf;
mod curve;
mod point;
mod compare;
pub mod boolean;
pub mod fileio;

pub use error::{BadArg, ErrorDetails, PsError, PsResult, Severity};
pub use boolean::{BooleanOp, BooleanOptions};
pub use memory::PkArray;
pub use frustrum::FrustrumConfig;
pub use session::{Behaviour, Mark, Session, SessionConfig, SmpInfo};
pub use partition::{Partition, Pmark, RollbackResult};
pub use entity::{Entity, PkClass};
pub use geom::{Axis2, Vec3};
pub use body::{Body, BodyType};
pub use mass::{MassProps, DEFAULT_MASS_ACCURACY};
pub use range::{Aabb, Enclosure};
pub use topology::{Fin, Loop, LoopType, Region, Shell};
pub use face::Face;
pub use edge::Edge;
pub use vertex::Vertex;
pub use surf::{Surf, SurfType, PlaneData, CylinderData, ConeData, SphereData, TorusData};
pub use surf::{SurfIntersection, IntersectionCurve};
pub use curve::{Curve, CurveType, LineData, CircleData, EllipseData};
pub use point::Point;
pub use compare::{SamplePoint, SurfaceParams, extract_surface_params};
