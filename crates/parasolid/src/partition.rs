//! Partition types — logical containers for entity ownership and rollback.
//!
//! Every entity belongs to exactly one partition. Partitions scope rollback
//! (undo/redo) and prevent cross-partition entity references.

use parasolid_sys::*;

use crate::error::PsResult;
use crate::memory::PkArray;
use crate::Entity;

// =============================================================================
// RollbackResult
// =============================================================================

/// Result of a rollback operation, tracking which entities changed.
#[derive(Debug)]
pub struct RollbackResult {
    /// Entities that were dead before rollback, alive after (resurrected).
    pub new_entities: Vec<Entity>,
    /// Entities that existed before and after, but were modified.
    pub modified_entities: Vec<Entity>,
    /// Entities that were alive before rollback, dead after.
    pub deleted_entities: Vec<Entity>,
}

// =============================================================================
// Pmark
// =============================================================================

/// A partition mark — a rollback checkpoint within a single partition.
///
/// Pmarks record the state of a partition at a point in time. Rolling back
/// to a pmark restores the partition to that state.
///
/// # Design note
///
/// `Pmark` carries no session lifetime (`'s`), for the same reasons as
/// [`Entity`](crate::Entity) and [`Partition`]: lifetime threading through
/// `Vec<Pmark>` return values was deemed too ergonomically burdensome for
/// v0.1. Using a pmark tag after its partition has been deleted or the session
/// has stopped is a PK-level error caught by argument checking in dev builds.
/// Revisit before v1.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pmark {
    tag: PK_PMARK_t,
}

impl Pmark {
    /// Wrap a raw PK pmark tag.
    pub(crate) fn from_tag(tag: PK_PMARK_t) -> Self {
        Pmark { tag }
    }

    /// Returns the raw PK tag for this pmark.
    #[inline]
    pub fn tag(&self) -> i32 {
        self.tag
    }

    /// Return the partition that owns this pmark.
    pub fn partition(&self) -> PsResult<Partition> {
        let mut tag: PK_PARTITION_t = 0;
        pk_call!(PK_PMARK_ask_partition(self.tag, &mut tag));
        Ok(Partition::from_tag(tag))
    }

    /// Roll the partition back to this pmark (ignoring change lists).
    ///
    /// `PK_PMARK_goto` writes the three counts *unconditionally*, so we must
    /// pass real (non-NULL) pointers even when discarding the results — NULL
    /// count pointers make the kernel fault. The returned arrays are freed.
    pub fn goto(&self) -> PsResult<()> {
        let mut n_new: i32 = 0;
        let mut new_p: *mut PK_ENTITY_t = std::ptr::null_mut();
        let mut n_mod: i32 = 0;
        let mut mod_p: *mut PK_ENTITY_t = std::ptr::null_mut();
        let mut n_del: i32 = 0;
        let mut del_p: *mut i32 = std::ptr::null_mut();
        pk_call!(PK_PMARK_goto(
            self.tag,
            &mut n_new, &mut new_p,
            &mut n_mod, &mut mod_p,
            &mut n_del, &mut del_p,
        ));
        // Free the kernel-allocated arrays via RAII.
        unsafe {
            let _ = PkArray::from_raw(new_p, n_new);
            let _ = PkArray::from_raw(mod_p, n_mod);
            let _ = PkArray::from_raw(del_p, n_del);
        }
        Ok(())
    }

    /// Roll the partition back to this pmark, tracking which entities changed.
    ///
    /// `PK_PMARK_goto` returns three arrays: `new_entities`/`mod_entities` as
    /// `PK_ENTITY_t` tags and `del_entities` as raw `int` tags (the entities are
    /// dead, so only their tag values remain).
    pub fn goto_with_tracking(&self) -> PsResult<RollbackResult> {
        let mut n_new: i32 = 0;
        let mut new_p: *mut PK_ENTITY_t = std::ptr::null_mut();
        let mut n_mod: i32 = 0;
        let mut mod_p: *mut PK_ENTITY_t = std::ptr::null_mut();
        let mut n_del: i32 = 0;
        let mut del_p: *mut i32 = std::ptr::null_mut();
        pk_call!(PK_PMARK_goto(
            self.tag,
            &mut n_new, &mut new_p,
            &mut n_mod, &mut mod_p,
            &mut n_del, &mut del_p,
        ));
        let new_entities = unsafe { PkArray::from_raw(new_p, n_new) }
            .iter()
            .map(|&tag| Entity::from_tag(tag))
            .collect();
        let modified_entities = unsafe { PkArray::from_raw(mod_p, n_mod) }
            .iter()
            .map(|&tag| Entity::from_tag(tag))
            .collect();
        let deleted_entities = unsafe { PkArray::from_raw(del_p, n_del) }
            .iter()
            .map(|&tag| Entity::from_tag(tag))
            .collect();
        Ok(RollbackResult {
            new_entities,
            modified_entities,
            deleted_entities,
        })
    }

    /// Return the pmark immediately preceding this one in the partition history.
    pub fn preceding(&self) -> PsResult<Pmark> {
        let mut tag: PK_PMARK_t = 0;
        pk_call!(PK_PMARK_ask_preceding(self.tag, &mut tag));
        Ok(Pmark::from_tag(tag))
    }

    /// Return the pmarks immediately following this one in the partition history.
    pub fn following(&self) -> PsResult<Vec<Pmark>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_PMARK_ask_following(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Pmark::from_tag(tag)).collect())
    }

    /// Return the identifier assigned to this pmark.
    pub fn identifier(&self) -> PsResult<i32> {
        let mut id = 0;
        pk_call!(PK_PMARK_ask_identifier(self.tag, &mut id));
        Ok(id)
    }
}

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

    /// Create a new partition. Requires partitioned rollback to be active
    /// (`SessionConfig::rollback(true)`), else `PK_ERROR_rollback_not_started`.
    /// Does not make the new partition current — call [`set_current`](Self::set_current).
    pub fn create() -> PsResult<Partition> {
        let opts = PK_PARTITION_create_o_t {
            o_t_version: 1,
            allow_partial_pmarks: PK_LOGICAL_false,
        };
        let mut res: PK_PARTITION_create_r_t = unsafe { std::mem::zeroed() };
        pk_call!(PK_PARTITION_create(&opts, &mut res));
        Ok(Partition { tag: res.partition })
    }

    /// Delete this partition and all entities it contains.
    ///
    /// The partition must not be the current partition.
    pub fn delete(self) -> PsResult<()> {
        let opts = PK_PARTITION_delete_o_t {
            o_t_version: 1,
            delete_non_empty: PK_LOGICAL_true,
        };
        pk_call!(PK_PARTITION_delete(self.tag, &opts));
        Ok(())
    }

    /// Create a rollback checkpoint (pmark) in this partition.
    pub fn make_pmark(&self) -> PsResult<Pmark> {
        let mut tag: PK_PMARK_t = 0;
        pk_call!(PK_PARTITION_make_pmark(self.tag, &mut tag));
        Ok(Pmark::from_tag(tag))
    }

    /// Return the current pmark of this partition and whether the partition is
    /// currently at that pmark.
    pub fn current_pmark(&self) -> PsResult<(Pmark, bool)> {
        let mut tag: PK_PMARK_t = 0;
        let mut is_at: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_PARTITION_ask_pmark(self.tag, &mut tag, &mut is_at));
        Ok((Pmark::from_tag(tag), is_at == PK_LOGICAL_true))
    }

    /// Return the initial pmark of this partition.
    pub fn initial_pmark(&self) -> PsResult<Pmark> {
        let mut tag: PK_PMARK_t = 0;
        pk_call!(PK_PARTITION_ask_initial_pmark(self.tag, &mut tag));
        Ok(Pmark::from_tag(tag))
    }

    /// Return all pmarks in this partition.
    pub fn pmarks(&self) -> PsResult<Vec<Pmark>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_PARTITION_ask_pmarks(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Pmark::from_tag(tag)).collect())
    }

    /// Advance the most recent pmark to the current state, returning it.
    pub fn advance_pmark(&self) -> PsResult<Pmark> {
        let opts = PK_PARTITION_advance_pmark_o_t {
            o_t_version: 1,
            _reserved: 0,
        };
        let mut tag: PK_PMARK_t = 0;
        pk_call!(PK_PARTITION_advance_pmark(self.tag, &opts, &mut tag));
        Ok(Pmark::from_tag(tag))
    }
}
