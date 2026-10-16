//! Validity checking — `PK_BODY_check`, `PK_FACE_check`, `PK_EDGE_check`,
//! `PK_FACE_check_pair`, `PK_GEOM_check`.
//!
//! The kernel's own consistency checker is the strongest single validity
//! signal for a body (an oracle-produced body, or one loaded from an external
//! XT). Each entry point has a plain form that passes NULL options (the kernel's
//! default check set) and a `_with` form that takes the full option struct.
//!
//! # ABI (audited 2026-10-16)
//!
//! * `PK_check_fault_t` is 40 bytes `{state, entity_1, entity_2, position}`.
//!   The old 12-byte binding misread every fault.
//! * Every option field after `max_faults` is a 4-byte enum token (not a
//!   logical). Current versions are BODY 5, FACE 4, EDGE 3, GEOM 3, pair 1.
//!   `Default` of each option struct equals the kernel's NULL-options defaults.
//!
//! Evidence: parasolid-re `catalog/pk-validated-abi.md` (P9 section and
//! re-validation table), `artifacts/port/oracle/runner/src/pkcheck.rs`. Tests:
//! `check_*` in `parasolid-test/src/abi_audit.rs`.
//!
//! **Residual risk.** Only the default token values and the `no` values of
//! `geom`/`top_geo` have been seen to change what is reported here; the meaning
//! of the `returns` / `extra_faults` levels is unprobed. The `position` field is
//! filled by the checker only for some fault states, so it may be zero.

use std::os::raw::c_int;

use parasolid_sys::*;

use crate::body::Body;
use crate::edge::Edge;
use crate::entity::Entity;
use crate::error::PsResult;
use crate::face::Face;
use crate::geom::Vec3;
use crate::memory::PkArray;

pub use parasolid_sys::{
    PK_BODY_check_o_t as BodyCheckOptions, PK_EDGE_check_o_t as EdgeCheckOptions,
    PK_FACE_check_o_t as FaceCheckOptions, PK_FACE_check_pair_o_t as FacePairCheckOptions,
    PK_GEOM_check_o_t as GeomCheckOptions,
};

/// A single fault reported by a check call.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckFault {
    /// The fault state token (a `PK_*_state_*_c` constant).
    pub state: i32,
    /// The entity where the fault was found, if the kernel names one.
    pub entity: Option<Entity>,
    /// A secondary entity related to the fault, if any.
    pub entity_2: Option<Entity>,
    /// The position the kernel attaches to the fault. Only meaningful for
    /// states that carry a location; otherwise whatever the kernel wrote
    /// (usually zero).
    pub position: Vec3,
}

impl CheckFault {
    pub(crate) fn from_pk(f: &PK_check_fault_t) -> Self {
        let ent = |t: PK_ENTITY_t| (t != PK_ENTITY_null).then(|| Entity::from_tag(t));
        CheckFault {
            state: f.state,
            entity: ent(f.entity_1),
            entity_2: ent(f.entity_2),
            position: Vec3::from_pk(f.position),
        }
    }
}

/// Collect a kernel-allocated fault array (freed on drop of the `PkArray`).
pub(crate) fn collect_faults(faults: *mut PK_check_fault_t, n: c_int) -> Vec<CheckFault> {
    let array = unsafe { PkArray::from_raw(faults, n) };
    array.iter().map(CheckFault::from_pk).collect()
}

impl Body {
    /// Run the kernel's default consistency checks on this body (NULL options).
    /// Returns the faults found — empty means the body is valid.
    pub fn check(&self) -> PsResult<Vec<CheckFault>> {
        self.check_impl(std::ptr::null_mut())
    }

    /// `PK_BODY_check` with an explicit option struct (version 5).
    pub fn check_with(&self, options: &BodyCheckOptions) -> PsResult<Vec<CheckFault>> {
        let mut o = *options;
        self.check_impl(&mut o)
    }

    fn check_impl(&self, options: *mut PK_BODY_check_o_t) -> PsResult<Vec<CheckFault>> {
        let mut n_faults: c_int = 0;
        let mut faults: *mut PK_check_fault_t = std::ptr::null_mut();
        pk_call!(PK_BODY_check(self.tag, options, &mut n_faults, &mut faults));
        Ok(collect_faults(faults, n_faults))
    }

    /// Convenience: `true` if [`Body::check`] finds no faults.
    pub fn is_valid(&self) -> PsResult<bool> {
        Ok(self.check()?.is_empty())
    }
}

impl Face {
    /// `PK_FACE_check` with the kernel's default checks (NULL options).
    pub fn check(&self) -> PsResult<Vec<CheckFault>> {
        self.check_impl(std::ptr::null())
    }

    /// `PK_FACE_check` with an explicit option struct (version 4).
    pub fn check_with(&self, options: &FaceCheckOptions) -> PsResult<Vec<CheckFault>> {
        self.check_impl(options)
    }

    fn check_impl(&self, options: *const PK_FACE_check_o_t) -> PsResult<Vec<CheckFault>> {
        let mut n: c_int = 0;
        let mut faults: *mut PK_check_fault_t = std::ptr::null_mut();
        pk_call!(PK_FACE_check(self.tag(), options, &mut n, &mut faults));
        Ok(collect_faults(faults, n))
    }

    /// `PK_FACE_check_pair`: face-face checks (e.g. clashes) between this face
    /// and `other`, kernel defaults.
    pub fn check_pair(&self, other: &Face) -> PsResult<Vec<CheckFault>> {
        self.check_pair_impl(other, std::ptr::null())
    }

    /// `PK_FACE_check_pair` with explicit options (version 1).
    pub fn check_pair_with(
        &self,
        other: &Face,
        options: &FacePairCheckOptions,
    ) -> PsResult<Vec<CheckFault>> {
        self.check_pair_impl(other, options)
    }

    fn check_pair_impl(
        &self,
        other: &Face,
        options: *const PK_FACE_check_pair_o_t,
    ) -> PsResult<Vec<CheckFault>> {
        let mut n: c_int = 0;
        let mut faults: *mut PK_check_fault_t = std::ptr::null_mut();
        pk_call!(PK_FACE_check_pair(
            self.tag(),
            other.tag(),
            options,
            &mut n,
            &mut faults
        ));
        Ok(collect_faults(faults, n))
    }
}

impl Edge {
    /// `PK_EDGE_check` with the kernel's default checks (NULL options).
    pub fn check(&self) -> PsResult<Vec<CheckFault>> {
        self.check_impl(std::ptr::null())
    }

    /// `PK_EDGE_check` with an explicit option struct (version 3).
    pub fn check_with(&self, options: &EdgeCheckOptions) -> PsResult<Vec<CheckFault>> {
        self.check_impl(options)
    }

    fn check_impl(&self, options: *const PK_EDGE_check_o_t) -> PsResult<Vec<CheckFault>> {
        let mut n: c_int = 0;
        let mut faults: *mut PK_check_fault_t = std::ptr::null_mut();
        pk_call!(PK_EDGE_check(self.tag(), options, &mut n, &mut faults));
        Ok(collect_faults(faults, n))
    }
}

/// `PK_GEOM_check` on any geometry tag (surface, curve or point), with the
/// kernel's default checks when `options` is `None`.
pub fn check_geom(geom: i32, options: Option<&GeomCheckOptions>) -> PsResult<Vec<CheckFault>> {
    let mut n: c_int = 0;
    let mut faults: *mut PK_check_fault_t = std::ptr::null_mut();
    let o = options.map_or(std::ptr::null(), |o| o as *const _);
    pk_call!(PK_GEOM_check(geom, o, &mut n, &mut faults));
    Ok(collect_faults(faults, n))
}
