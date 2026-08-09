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
mod attrib;
mod body;
pub mod boolean;
mod check;
mod compare;
mod create;
mod curve;
mod edge;
mod enclosure;
mod entity;
mod face;
mod facet;
pub mod fileio;
mod frustrum;
mod geom;
mod intersect;
mod jet;
mod mass;
mod memory;
pub mod oracle;
mod partition;
mod point;
mod range;
mod rollback;
mod session;
mod surf;
mod topology;
mod transform;
mod vertex;

pub use attrib::{ATTRIB_COLOUR, AttribDef};
pub use body::{Body, BodyType};
pub use boolean::{BooleanOp, BooleanOptions};
pub use check::CheckFault;
pub use compare::{SamplePoint, SurfaceParams, extract_surface_params};
pub use curve::{
    BCurveData, CurveCurvature, CurveParam, ParamCurveClass, ParamExtent, Periodicity,
};
pub use curve::{CircleData, Curve, CurveType, EllipseData, LineData};
pub use edge::{Edge, EdgeType};
pub use enclosure::OrientedBox;
pub use entity::{
    ClashRecord, Entity, GeomCategory, Obb, PkClass, RangeResult, RangeStatus, RangeWitness,
};
pub use error::{BadArg, ErrorDetails, PsError, PsResult, Severity};
pub use face::{Coincidence, Face};
pub use facet::{FacetMesh, Mesh};
pub use frustrum::FrustrumConfig;
pub use geom::{Axis2, Vec3};
pub use intersect::{CurveCurveHit, FaceCurveHit, SurfCurveHit};
pub use jet::{CurveJet, Hand, MinRadius, SurfJet};
pub use mass::{DEFAULT_MASS_ACCURACY, MassProps};
pub use memory::PkArray;
pub use oracle::{CurveSample, SurfaceSample, TopologySummary};
pub use partition::{Partition, Pmark, RollbackResult};
pub use point::Point;
pub use range::{Aabb, Enclosure};
pub use session::{Behaviour, Mark, Session, SessionConfig, SmpInfo};
pub use surf::{ConeData, CylinderData, PlaneData, SphereData, Surf, SurfType, TorusData, UvBox};
pub use surf::{IntersectionCurve, IntersectionKind, SurfIntersection};
pub use surf::{OffsetData, SpunData, SurfCurvature, SurfDirParam, SweptData};
pub use topology::{Fin, FinType, Loop, LoopType, Region, RegionType, Shell, ShellSign, ShellType};
pub use transform::{Classification, MatrixType, Transform};
pub use vertex::{Vertex, VertexType};
