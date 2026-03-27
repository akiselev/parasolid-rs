//! Partition types — logical containers for entity ownership and rollback.
//!
//! Every entity belongs to exactly one partition. Partitions scope rollback
//! (undo/redo) and prevent cross-partition entity references.

use parasolid_sys::*;

use crate::error::PsResult;
use crate::memory::PkArray;
use crate::Entity;

/// A Parasolid partition — a logical grouping of entities.
///
/// Partitions are created automatically when a session starts (one initial
/// partition) and can be created/deleted/merged during the session.
///
/// Entities within a partition can reference each other, but cross-partition
/// references are forbidden. Rollback is scoped to individual partitions
/// (via partition marks) or the entire session (via session marks).
///
/// # Design note
///
/// `Partition` carries no session lifetime (`'s`), for the same reasons as
/// [`Entity`](crate::Entity): lifetime threading through `Vec<Partition>`
/// return values was deemed too ergonomically burdensome for v0.1. After
/// `delete(self)`, the tag becomes invalid; using it is a PK-level error
/// caught by argument checking in dev builds. Revisit before v1.0 alongside
/// `Entity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Partition {
    tag: PK_PARTITION_t,
}

impl Partition {
    /// Wrap a raw partition tag.
    pub(crate) fn from_tag(tag: PK_PARTITION_t) -> Self {
        Partition { tag }
    }

    /// Returns the raw PK tag for this partition.
    #[inline]
    pub fn tag(&self) -> i32 {
        self.tag
    }

    /// Returns all bodies in this partition.
    pub fn bodies(&self) -> PsResult<Vec<Entity>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_PARTITION_ask_bodies(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Entity::from_tag(tag)).collect())
    }

    /// Returns all geometric entities (surfaces, curves, points) in this
    /// partition that are not attached to any body ("orphan geometry").
    pub fn geoms(&self) -> PsResult<Vec<Entity>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_PARTITION_ask_geoms(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Entity::from_tag(tag)).collect())
    }

    /// Make this partition the current partition for the session.
    ///
    /// New entities will be created in the current partition.
    pub fn set_current(&self) -> PsResult<()> {
        pk_call!(PK_PARTITION_set_current(self.tag));
        Ok(())
    }

    /// Create a new empty partition.
    pub fn create() -> PsResult<Partition> {
        let mut tag: PK_PARTITION_t = 0;
        pk_call!(PK_PARTITION_create(&mut tag));
        Ok(Partition { tag })
    }

    /// Delete this partition and all entities it contains.
    ///
    /// The partition must not be the current partition.
    pub fn delete(self) -> PsResult<()> {
        let opts = PK_PARTITION_delete_o_t {
            delete_non_empty: PK_LOGICAL_true,
        };
        pk_call!(PK_PARTITION_delete(1, &self.tag, &opts));
        Ok(())
    }
}
