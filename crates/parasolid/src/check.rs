//! Body validity checking — `PK_BODY_check`.
//!
//! The kernel's own consistency checker is the strongest single validity
//! signal for a body loaded from an external source (e.g. an XT file from a
//! dataset). [`Body::check`] runs the default comprehensive set of checks and
//! returns any faults found; an empty result means the body is valid.

use std::os::raw::c_int;

use parasolid_sys::*;

use crate::body::Body;
use crate::entity::Entity;
use crate::error::PsResult;
use crate::memory::PkArray;

/// A single fault reported by [`Body::check`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckFault {
    /// The entity where the fault was found.
    pub entity: Entity,
    /// The fault state token (a `PK_*_state_*_c` constant); never
    /// `PK_BODY_state_ok_c`, since only actual faults are reported.
    pub state: i32,
    /// A secondary entity related to the fault, if any.
    pub entity_2: Option<Entity>,
}

impl Body {
    /// Run the kernel's default consistency checks on this body.
    ///
    /// Returns the list of faults found — empty means the body is valid. Passes
    /// `NULL` options so the kernel applies its default comprehensive check set.
    pub fn check(&self) -> PsResult<Vec<CheckFault>> {
        let mut n_faults: c_int = 0;
        let mut faults: *mut PK_check_fault_t = std::ptr::null_mut();
        pk_call!(PK_BODY_check(
            self.tag,
            std::ptr::null_mut(),
            &mut n_faults,
            &mut faults,
        ));
        let array = unsafe { PkArray::from_raw(faults, n_faults) };
        Ok(array
            .iter()
            .map(|f| CheckFault {
                entity: Entity::from_tag(f.entity),
                state: f.state,
                entity_2: (f.entity_2 != PK_ENTITY_null).then(|| Entity::from_tag(f.entity_2)),
            })
            .collect())
    }

    /// Convenience: `true` if [`Body::check`] finds no faults.
    pub fn is_valid(&self) -> PsResult<bool> {
        Ok(self.check()?.is_empty())
    }
}
