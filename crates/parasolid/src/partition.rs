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

    /// Roll the partition back to this pmark.
    pub fn goto(&self) -> PsResult<()> {
        pk_call!(PK_PMARK_goto(self.tag));
        Ok(())
    }

    /// Roll the partition back to this pmark, tracking which entities changed.
    pub fn goto_with_tracking(&self) -> PsResult<RollbackResult> {
        let opts = PK_PMARK_goto_2_o_t {
            want_new_entities: PK_LOGICAL_true,
            want_mod_entities: PK_LOGICAL_true,
            want_del_entities: PK_LOGICAL_true,
            want_logged_mod: PK_LOGICAL_false,
            want_attrib_mod: PK_LOGICAL_false,
            del_attrib_cb: None,
            del_context: std::ptr::null_mut(),
            n_del_attdefs: 0,
            del_attdefs: std::ptr::null(),
            n_new_entities_classes: 0,
            new_entities_classes: std::ptr::null(),
            n_mod_entities_classes: 0,
            mod_entities_classes: std::ptr::null(),
            n_del_entities_classes: 0,
            del_entities_classes: std::ptr::null(),
            no_roll_diff: PK_LOGICAL_false,
        };
        let mut result: PK_PMARK_goto_2_r_t = unsafe { std::mem::zeroed() };
        pk_call!(PK_PMARK_goto_2(self.tag, &opts, &mut result));
        let new_entities = unsafe { PkArray::from_raw(result.new_entities, result.n_new_entities) }
            .iter()
            .map(|&tag| Entity::from_tag(tag))
            .collect();
        let modified_entities =
            unsafe { PkArray::from_raw(result.mod_entities, result.n_mod_entities) }
                .iter()
                .map(|&tag| Entity::from_tag(tag))
                .collect();
        let deleted_entities =
            unsafe { PkArray::from_raw(result.del_entities, result.n_del_entities) }
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

    /// Advance the most recent pmark to the current state.
    pub fn advance_pmark(&self) -> PsResult<()> {
        pk_call!(PK_PARTITION_advance_pmark(self.tag));
        Ok(())
    }
}
