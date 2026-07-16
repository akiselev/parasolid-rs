//! Partition, partition-mark (PMARK), session-mark (MARK), and delta bindings.
//!
//! Covers partition management, entity/partition queries, partition marks,
//! session marks, deltas, and rollback functions from Parasolid chapters 94-95.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_int, c_void};

use crate::*;

// =============================================================================
// Partition type enum
// =============================================================================

/// Partition type: standard (full delta history) or light (no backward delta
/// at first non-initial pmark).
pub type PK_partition_type_t = c_int;
// [re-abi] appended 2 missing member(s) from pk-enums.h
pub const PK_PARTITION_type_standard_c: PK_partition_type_t = 23510;
pub const PK_PARTITION_type_light_c: PK_partition_type_t = 23511;



// =============================================================================
// PK_PARTITION_copy — copy_deltas enum
// =============================================================================

/// Controls which deltas are copied during `PK_PARTITION_copy`.
pub type PK_partition_copy_deltas_t = c_int;
// [re-abi] appended 4 missing member(s) from pk-enums.h
pub const PK_PARTITION_copy_deltas_none_c: PK_partition_copy_deltas_t = 23310;
pub const PK_PARTITION_copy_deltas_all_c: PK_partition_copy_deltas_t = 23311;
pub const PK_PARTITION_copy_deltas_main_c: PK_partition_copy_deltas_t = 23312;
pub const PK_PARTITION_copy_deltas_curr_c: PK_partition_copy_deltas_t = 23313;





// =============================================================================
// PK_PARTITION_create options + results
// =============================================================================

/// Options for `PK_PARTITION_create`. Layout `o_t_version@0,
/// allow_partial_pmarks@4` (8 bytes) per the V35 docs / RE catalog.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARTITION_create_o_t {
    pub o_t_version: c_int,
    pub allow_partial_pmarks: PK_LOGICAL_t,
}

/// Results of `PK_PARTITION_create` — the new partition tag.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARTITION_create_r_t {
    pub partition: PK_PARTITION_t,
}

// =============================================================================
// PK_PARTITION_advance_pmark options
// =============================================================================

/// Options for `PK_PARTITION_advance_pmark` (v1: just the version field).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARTITION_advance_pmark_o_t {
    pub o_t_version: c_int,
    /// Reserved (`number`/version migration) — keeps the documented 8-byte size.
    pub _reserved: c_int,
}

// =============================================================================
// PK_PARTITION_delete options
// =============================================================================

/// Options for `PK_PARTITION_delete`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARTITION_delete_o_t {
    /// Structure version (offset 0). The old struct omitted this, so the kernel
    /// read `delete_non_empty` as the version and the real flag from past the
    /// allocation. Layout: `o_t_version@0, delete_non_empty@4` (8 bytes).
    pub o_t_version: c_int,
    /// If `PK_LOGICAL_true`, non-empty partitions can be deleted with all data.
    pub delete_non_empty: PK_LOGICAL_t,
}

// =============================================================================
// PK_PARTITION_copy options
// =============================================================================

/// Options for `PK_PARTITION_copy`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARTITION_copy_o_t {
    /// Controls which deltas are copied.
    pub copy_deltas: PK_partition_copy_deltas_t,
}

// =============================================================================
// PK_PARTITION_make_pmark_2 options
// =============================================================================

/// Options for `PK_PARTITION_make_pmark_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARTITION_make_pmark_2_o_t {
    /// Request entity change information.
    pub want_entity_changes: PK_LOGICAL_t,
}

// =============================================================================
// PK_PMARK_goto_2 options
// =============================================================================

/// Callback invoked before attribute deletion during rollback.
pub type PK_PMARK_goto_del_attrib_cb_t = Option<
    unsafe extern "C" fn(
        attrib: PK_ATTRIB_t,
        context: *mut c_void,
    ),
>;

/// Options for `PK_PMARK_goto_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PMARK_goto_2_o_t {
    /// Return new entities array.
    pub want_new_entities: PK_LOGICAL_t,
    /// Return modified entities array.
    pub want_mod_entities: PK_LOGICAL_t,
    /// Return deleted entities array.
    pub want_del_entities: PK_LOGICAL_t,
    /// Only report entities with modeling changes.
    pub want_logged_mod: PK_LOGICAL_t,
    /// Control attribute modification reporting.
    pub want_attrib_mod: PK_LOGICAL_t,
    /// Callback called before attribute deletion.
    pub del_attrib_cb: PK_PMARK_goto_del_attrib_cb_t,
    /// Context passed to `del_attrib_cb`.
    pub del_context: *mut c_void,
    /// Number of attdefs to limit `del_attrib_cb` to.
    pub n_del_attdefs: c_int,
    /// Limit `del_attrib_cb` to specific attdefs.
    pub del_attdefs: *const PK_ATTDEF_t,
    /// Number of class filters for new entities.
    pub n_new_entities_classes: c_int,
    /// Filter new entities by class.
    pub new_entities_classes: *const PK_CLASS_t,
    /// Number of class filters for modified entities.
    pub n_mod_entities_classes: c_int,
    /// Filter modified entities by class.
    pub mod_entities_classes: *const PK_CLASS_t,
    /// Number of class filters for deleted entities.
    pub n_del_entities_classes: c_int,
    /// Filter deleted entities by class.
    pub del_entities_classes: *const PK_CLASS_t,
    /// Report no-roll attribute field changes.
    pub no_roll_diff: PK_LOGICAL_t,
}

// =============================================================================
// PK_PMARK_goto_2 return struct
// =============================================================================

/// Return data from `PK_PMARK_goto_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PMARK_goto_2_r_t {
    /// Number of entities created by the rollback.
    pub n_new_entities: c_int,
    /// Entities that were dead before roll, alive after.
    pub new_entities: *mut PK_ENTITY_t,
    /// Number of entities modified by the rollback.
    pub n_mod_entities: c_int,
    /// Entities existing before and after, possibly modified.
    pub mod_entities: *mut PK_ENTITY_t,
    /// Number of entities deleted by the rollback.
    pub n_del_entities: c_int,
    /// Entities alive before roll, dead after.
    pub del_entities: *mut PK_ENTITY_t,
}

// =============================================================================
// PK_MARK_goto_2 options
// =============================================================================

/// Options for `PK_MARK_goto_2` (same structure as `PK_PMARK_goto_2_o_t`).
pub type PK_MARK_goto_2_o_t = PK_PMARK_goto_2_o_t;

/// Return data from `PK_MARK_goto_2`.
pub type PK_MARK_goto_2_r_t = PK_PMARK_goto_2_r_t;

/// Options for `PK_MARK_start` — a single `forward` flag (no version field).
/// Layout `forward:PK_LOGICAL_t@0` (4 bytes) per the RE catalog.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MARK_start_o_t {
    pub forward: PK_LOGICAL_t,
}

// =============================================================================
// PK_PMARK_ask_entities return struct
// =============================================================================

/// Return data from `PK_PMARK_ask_entities` (what-if query).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PMARK_ask_entities_r_t {
    /// Number of entities that would be created.
    pub n_new_entities: c_int,
    /// Entities that would be created if rolled to pmark.
    pub new_entities: *mut PK_ENTITY_t,
    /// Number of entities that would be modified.
    pub n_mod_entities: c_int,
    /// Entities that would be modified if rolled to pmark.
    pub mod_entities: *mut PK_ENTITY_t,
    /// Number of entities that would be deleted.
    pub n_del_entities: c_int,
    /// Entities that would be deleted if rolled to pmark.
    pub del_entities: *mut PK_ENTITY_t,
}

// =============================================================================
// Delta Frustrum callback types (registered via PK_DELTA_register_callbacks)
// =============================================================================

// ABI recovered by decompiling the kernel-side callers (PKF_delta_*, RE
// c900fa3f430f): every callback RETURNS its `ifail` code (0 = success) and
// takes scalars BY VALUE — there is NO trailing `*ifail` out-param, and `read`
// has no `actual_bytes` out-param (the kernel tracks exact block sizes).

/// Delta callback: open/create a delta stream for `pmark`, returning its stream
/// id in `*strid`. Returns ifail (0 = success). Kernel caller: `(tag, strid*)`.
pub type PK_DELTA_open_for_write_f_t =
    Option<unsafe extern "C" fn(pmark: PK_PMARK_t, strid: *mut c_int) -> c_int>;

/// Delta callback: open the delta stream for `pmark` for reading. Unlike the
/// write side, this takes ONLY the pmark (no strid out-param): read streams are
/// identified by the pmark itself, which the kernel then passes to `read`/`close`.
pub type PK_DELTA_open_for_read_f_t =
    Option<unsafe extern "C" fn(pmark: PK_PMARK_t) -> c_int>;

/// Delta callback: close a delta stream. Returns ifail.
pub type PK_DELTA_close_f_t = Option<unsafe extern "C" fn(strid: c_int) -> c_int>;

/// Delta callback: write `n_bytes` from `buffer` to the stream. Returns ifail.
pub type PK_DELTA_write_f_t =
    Option<unsafe extern "C" fn(strid: c_int, n_bytes: c_int, buffer: *const c_void) -> c_int>;

/// Delta callback: read `max_bytes` into `buffer` from the stream. Returns ifail.
pub type PK_DELTA_read_f_t =
    Option<unsafe extern "C" fn(strid: c_int, max_bytes: c_int, buffer: *mut c_void) -> c_int>;

/// Delta callback: delete the delta stored for `pmark`. Returns ifail.
pub type PK_DELTA_delete_f_t = Option<unsafe extern "C" fn(pmark: PK_PMARK_t) -> c_int>;

/// Frustrum delta callbacks structure for `PK_DELTA_register_callbacks`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_DELTA_callbacks_t {
    pub open_for_write_fn: PK_DELTA_open_for_write_f_t,
    pub open_for_read_fn: PK_DELTA_open_for_read_f_t,
    pub close_fn: PK_DELTA_close_f_t,
    pub write_fn: PK_DELTA_write_f_t,
    pub read_fn: PK_DELTA_read_f_t,
    pub delete_fn: PK_DELTA_delete_f_t,
}

// =============================================================================
// No-roll diff constants
// =============================================================================

/// Report type for no-roll diff.
pub const PK_REPORT_1_no_roll_diff_c: c_int = 23927;

// =============================================================================
// Null constants
// =============================================================================

/// Null partition mark value.
pub const PK_PMARK_null: PK_PMARK_t = 0;

// =============================================================================
// Foreign functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // =========================================================================
    // Partition management
    // =========================================================================

    /// Create a new partition in the session. Requires partitioned rollback to
    /// be active (`PK_DELTA_register_callbacks` before `PK_SESSION_start`), else
    /// `PK_ERROR_rollback_not_started`. Does NOT make the new partition current.
    /// V35: `(const PK_PARTITION_create_o_t *options, PK_PARTITION_create_r_t *results)`.
    pub fn PK_PARTITION_create(
        options: *const PK_PARTITION_create_o_t,
        results: *mut PK_PARTITION_create_r_t,
    ) -> PK_ERROR_code_t;

    /// Create a new empty partition in the session.
    pub fn PK_PARTITION_create_empty(
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Set partition type (light or standard).
    pub fn PK_PARTITION_set_type(
        partition: PK_PARTITION_t,
        type_: PK_partition_type_t,
    ) -> PK_ERROR_code_t;

    /// Ask partition type.
    pub fn PK_PARTITION_ask_type(
        partition: PK_PARTITION_t,
        type_: *mut PK_partition_type_t,
    ) -> PK_ERROR_code_t;

    /// Copy an entire partition (with options for delta copying).
    pub fn PK_PARTITION_copy(
        partition: PK_PARTITION_t,
        options: *const PK_PARTITION_copy_o_t,
        copy: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Delete a partition and its contents. V35: `(PK_PARTITION_t partition,
    /// const PK_PARTITION_delete_o_t *options)` — a SINGLE partition, not an array.
    pub fn PK_PARTITION_delete(
        partition: PK_PARTITION_t,
        options: *const PK_PARTITION_delete_o_t,
    ) -> PK_ERROR_code_t;

    /// Merge two or more partitions together.
    pub fn PK_PARTITION_merge(
        n_partitions: c_int,
        partitions: *mut PK_PARTITION_t,
        n_pmarks: c_int,
        pmarks: *mut PK_PMARK_t,
        options: *mut PK_PARTITION_merge_o_t,
    ) -> PK_ERROR_code_t;

    /// Set the current partition.
    pub fn PK_PARTITION_set_current(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Return whether a tag is a partition.
    pub fn PK_PARTITION_is(
        entity: PK_ENTITY_t,
        is_partition: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the set of appitems in the given partition.
    pub fn PK_PARTITION_ask_appitems(
        partition: PK_PARTITION_t,
        n_appitems: *mut c_int,
        appitems: *mut *mut PK_APPITEM_t,
    ) -> PK_ERROR_code_t;

    /// Return all bodies in the partition.
    pub fn PK_PARTITION_ask_bodies(
        partition: PK_PARTITION_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return all assemblies in the partition.
    pub fn PK_PARTITION_ask_assemblies(
        partition: PK_PARTITION_t,
        n_assemblies: *mut c_int,
        assemblies: *mut *mut PK_ASSEMBLY_t,
    ) -> PK_ERROR_code_t;

    /// Return all geometric entities in the partition.
    pub fn PK_PARTITION_ask_geoms(
        partition: PK_PARTITION_t,
        n_geoms: *mut c_int,
        geoms: *mut *mut PK_GEOM_t,
    ) -> PK_ERROR_code_t;

    /// Return all transforms in the partition.
    pub fn PK_PARTITION_ask_transfs(
        partition: PK_PARTITION_t,
        n_transfs: *mut c_int,
        transfs: *mut *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Entity/partition queries
    // =========================================================================

    /// Move a (new) body into a different partition.
    pub fn PK_BODY_change_partition(
        body: PK_BODY_t,
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Partition marks (PMARK)
    // =========================================================================

    /// Returns true if partition mark exists.
    pub fn PK_PMARK_is(
        entity: PK_ENTITY_t,
        is_pmark: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Delete partition marks.
    pub fn PK_PMARK_delete(
        n_pmarks: c_int,
        pmarks: *mut PK_PMARK_t,
        n_bad_pmarks: *mut c_int,
        bad_pmarks: *mut *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Return partition of a partition mark.
    pub fn PK_PMARK_ask_partition(
        pmark: PK_PMARK_t,
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Return session marks using the given pmark.
    pub fn PK_PMARK_ask_marks(
        pmark: PK_PMARK_t,
        n_marks: *mut c_int,
        marks: *mut *mut PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Returns true if pmark is used by a session mark.
    pub fn PK_PMARK_is_used_by_mark(
        pmark: PK_PMARK_t,
        is_used: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return pmarks immediately following given one.
    pub fn PK_PMARK_ask_following(
        pmark: PK_PMARK_t,
        n_following: *mut c_int,
        following: *mut *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Return partition mark preceding given one.
    pub fn PK_PMARK_ask_preceding(
        pmark: PK_PMARK_t,
        preceding: *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Return the identifier of a pmark.
    pub fn PK_PMARK_ask_identifier(
        pmark: PK_PMARK_t,
        identifier: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// What-if query: entities created/deleted/modified if rolled to pmark.
    pub fn PK_PMARK_ask_entities(
        pmark: PK_PMARK_t,
        options: *mut PK_PMARK_ask_entities_o_t,
        n_new: *mut c_int,
        new_entities: *mut *mut c_int,
        n_mod: *mut c_int,
        mod_entities: *mut *mut c_int,
        n_del: *mut c_int,
        del_entities: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Roll partition to specified pmark, reporting entity changes. All output
    /// pointers are optional (may be NULL). V35: `(PK_PMARK_t pmark, int *n_new,
    /// PK_ENTITY_t **new_entities, int *n_mod, PK_ENTITY_t **mod_entities,
    /// int *n_del, int **del_entities)`. Note `del_entities` is `int**` (tags of
    /// now-dead entities), while new/mod are `PK_ENTITY_t**`.
    pub fn PK_PMARK_goto(
        pmark: PK_PMARK_t,
        n_new: *mut c_int,
        new_entities: *mut *mut PK_ENTITY_t,
        n_mod: *mut c_int,
        mod_entities: *mut *mut PK_ENTITY_t,
        n_del: *mut c_int,
        del_entities: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Roll partition to a pmark with options (class filters, attrib callbacks).
    /// V35: 8 args with INDIVIDUAL out-params (not a results struct):
    /// `(PK_PMARK_t pmark, const PK_PMARK_goto_o_t *options, int *n_new,
    /// PK_ENTITY_t **new_entities, int *n_mod, PK_ENTITY_t **mod_entities,
    /// int *n_del, int **del_entities)`.
    pub fn PK_PMARK_goto_2(
        pmark: PK_PMARK_t,
        options: *const PK_PMARK_goto_2_o_t,
        n_new: *mut c_int,
        new_entities: *mut *mut PK_ENTITY_t,
        n_mod: *mut c_int,
        mod_entities: *mut *mut PK_ENTITY_t,
        n_del: *mut c_int,
        del_entities: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Return current pmark of partition and whether at that pmark.
    pub fn PK_PARTITION_ask_pmark(
        partition: PK_PARTITION_t,
        pmark: *mut PK_PMARK_t,
        is_at_pmark: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return initial pmark of partition.
    pub fn PK_PARTITION_ask_initial_pmark(
        partition: PK_PARTITION_t,
        pmark: *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Return all pmarks in partition.
    pub fn PK_PARTITION_ask_pmarks(
        partition: PK_PARTITION_t,
        n_pmarks: *mut c_int,
        pmarks: *mut *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Return all pmarks in partition (version 2, with rollback-forward support).
    pub fn PK_PARTITION_ask_pmarks_2(
        partition: PK_PARTITION_t,
        n_pmarks: *mut c_int,
        pmarks: *mut *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Return bytes that would be written to Frustrum if pmark set.
    pub fn PK_PARTITION_ask_pmark_size(
        partition: PK_PARTITION_t,
        size: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Create pmark in partition (with entity change info).
    pub fn PK_PARTITION_make_pmark_2(
        partition: PK_PARTITION_t,
        options: *mut PK_PARTITION_make_pmark_o_t,
        pmark: *mut PK_PMARK_t,
        n_new: *mut c_int,
        new_entities: *mut *mut PK_ENTITY_t,
        n_mod: *mut c_int,
        mod_entities: *mut *mut PK_ENTITY_t,
        n_del: *mut c_int,
        del_entities: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Create pmark in partition.
    pub fn PK_PARTITION_make_pmark(
        partition: PK_PARTITION_t,
        pmark: *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Return pmark with specified identifier.
    pub fn PK_PARTITION_find_pmark_by_id(
        partition: PK_PARTITION_t,
        identifier: c_int,
        pmark: *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Advance most recent pmark to current state. V35: `(PK_PARTITION_t
    /// partition, const PK_PARTITION_advance_pmark_o_t *options, PK_PMARK_t *pmark)`.
    pub fn PK_PARTITION_advance_pmark(
        partition: PK_PARTITION_t,
        options: *const PK_PARTITION_advance_pmark_o_t,
        pmark: *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Clone a pmark.
    pub fn PK_PARTITION_clone_pmark(
        partition: PK_PARTITION_t,
        pmark: PK_PMARK_t,
        clone: *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Session marks (MARK)
    // =========================================================================

    /// Returns true if session mark exists.
    pub fn PK_MARK_is(
        entity: PK_ENTITY_t,
        is_mark: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Create a session mark (creates pmarks in all partitions not at a pmark).
    pub fn PK_MARK_create(
        mark: *mut PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Create a session mark (version 2).
    pub fn PK_MARK_create_2(
        mark: *mut PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Delete a session mark.
    pub fn PK_MARK_delete(
        mark: PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Delete a session mark (version 2).
    pub fn PK_MARK_delete_2(
        mark: PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Roll session to state when given mark was set (legacy).
    pub fn PK_MARK_goto(
        mark: PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Roll session to state when given mark was set (with options and return data).
    pub fn PK_MARK_goto_2(
        mark: PK_MARK_t,
        options: *const PK_MARK_goto_2_o_t,
        result: *mut PK_MARK_goto_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Return mark after the given one.
    pub fn PK_MARK_ask_following(
        mark: PK_MARK_t,
        following: *mut PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Return mark before the given one.
    pub fn PK_MARK_ask_preceding(
        mark: PK_MARK_t,
        preceding: *mut PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Return pmarks that would be current if mark rolled to.
    pub fn PK_MARK_ask_pmarks(
        mark: PK_MARK_t,
        n_pmarks: *mut c_int,
        pmarks: *mut *mut PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Current state of non-partitioned PK rollback (deprecated; superseded by
    /// `PK_SESSION_ask_mark`). V35: `(PK_MARK_t *current, PK_LOGICAL_t *is_at_mark)`
    /// — BOTH are out-params (the old binding wrongly took `mark` by value).
    pub fn PK_MARK_ask_state(
        current: *mut PK_MARK_t,
        is_at_mark: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return whether mark rollback is active.
    pub fn PK_MARK_is_on(
        is_on: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Whether roll-forward is enabled. V35: `(PK_LOGICAL_t *is_enabled)` — a
    /// single out-param (only valid once non-partitioned rollback is started).
    pub fn PK_MARK_ask_forward(
        is_enabled: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the registered mark frustrum. V35: `(PK_MARK_frustrum_t *frustrum)`.
    pub fn PK_MARK_ask_frustrum(
        frustrum: *mut PK_MARK_frustrum_t,
    ) -> PK_ERROR_code_t;

    /// Start non-partitioned PK rollback. V35: `(PK_MARK_frustrum_t frustrum,
    /// const PK_MARK_start_o_t *options)`. Mutually exclusive with partitioned
    /// rollback (returns `PK_ERROR_rollback_started` if that is already active).
    pub fn PK_MARK_start(
        frustrum: PK_MARK_frustrum_t,
        options: *const PK_MARK_start_o_t,
    ) -> PK_ERROR_code_t;

    /// Stop mark-based rollback (legacy).
    pub fn PK_MARK_stop() -> PK_ERROR_code_t;

    // =========================================================================
    // Delta management
    // =========================================================================

    /// Register rollback Frustrum functions (must call before session start).
    pub fn PK_DELTA_register_callbacks(
        callbacks: *const PK_DELTA_callbacks_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Partition I/O
    // =========================================================================

    /// Receive (import) a partition from Frustrum.
    pub fn PK_PARTITION_receive(
        key: *mut c_char,
        options: *mut PK_PARTITION_receive_o_t,
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Receive partition (binary format).
    /// V35: `(PK_MEMORY_block_t block, options, PK_PARTITION_t *partition)`.
    pub fn PK_PARTITION_receive_b(
        block: PK_MEMORY_block_t,
        options: *const PK_PARTITION_receive_o_t,
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Receive partition (Unicode format).
    pub fn PK_PARTITION_receive_u(
        key: *mut PK_UCHAR_t,
        options: *mut PK_PARTITION_receive_o_t,
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Transmit (export) a partition to Frustrum.
    pub fn PK_PARTITION_transmit(
        partition: PK_PARTITION_t,
        key: *mut c_char,
        options: *mut PK_PARTITION_transmit_o_t,
    ) -> PK_ERROR_code_t;

    /// Transmit partition (binary format).
    pub fn PK_PARTITION_transmit_b(
        partition: PK_PARTITION_t,
        options: *mut PK_PARTITION_transmit_o_t,
        block: *mut PK_MEMORY_block_t,
        deltas: *mut PK_MEMORY_block_t,
    ) -> PK_ERROR_code_t;

    /// Transmit partition (Unicode format).
    pub fn PK_PARTITION_transmit_u(
        partition: PK_PARTITION_t,
        key: *mut PK_UCHAR_t,
        options: *mut PK_PARTITION_transmit_o_t,
    ) -> PK_ERROR_code_t;

    /// Transmit a delta between pmarks.
    /// [RE-regenerated from V35 TSV prototype]
    pub fn PK_PARTITION_transmit_delta(
        partition: PK_PARTITION_t,
        options: *mut PK_PARTITION_transmit_delta_o_t,
    ) -> PK_ERROR_code_t;

    /// Receive deltas for a partition.
    pub fn PK_PARTITION_receive_deltas(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Receive deltas for a partition (version 2).
    pub fn PK_PARTITION_receive_deltas_2(
        partition: PK_PARTITION_t,
        options: *mut PK_PARTITION_receive_deltas_o_t,
    ) -> PK_ERROR_code_t;

    /// Receive partition version information.
    pub fn PK_PARTITION_receive_version(
        key: *mut c_char,
        transmit_format: PK_transmit_format_t,
        version: *mut PK_SESSION_kernel_version_t,
    ) -> PK_ERROR_code_t;

    /// Receive partition version (binary format).
    /// V35: `(PK_MEMORY_block_t block, PK_transmit_format_t transmit_format, PK_SESSION_kernel_version_t *version)`.
    pub fn PK_PARTITION_receive_version_b(
        block: PK_MEMORY_block_t,
        transmit_format: PK_transmit_format_t,
        version: *mut PK_SESSION_kernel_version_t,
    ) -> PK_ERROR_code_t;

    /// Receive partition version (Unicode format).
    pub fn PK_PARTITION_receive_version_u(
        key: *mut PK_UCHAR_t,
        transmit_format: PK_transmit_format_t,
        version: *mut PK_SESSION_kernel_version_t,
    ) -> PK_ERROR_code_t;

    /// Receive mesh data into partition.
    pub fn PK_PARTITION_receive_meshes(
        partition: PK_PARTITION_t,
        options: *mut PK_PARTITION_receive_meshes_o_t,
        n_owners: *mut c_int,
        owners: *mut *mut PK_ITEM_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Partition guard
    // =========================================================================

    /// Set a guard on a partition.
    pub fn PK_PARTITION_set_guard(
        partition: PK_PARTITION_t,
        pmark: PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Check if a partition has a guard.
    pub fn PK_PARTITION_has_guard(
        partition: PK_PARTITION_t,
        has_guard: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Roll partition to its guard pmark.
    pub fn PK_PARTITION_goto_guard(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Clear a guard on a partition.
    pub fn PK_PARTITION_clear_guard(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Partition cloning
    // =========================================================================

    /// Start cloning mode for a partition.
    pub fn PK_PARTITION_start_cloning(
        partition: PK_PARTITION_t,
        from_pmark: PK_PMARK_t,
        to_pmark: PK_PMARK_t,
        options: *mut PK_PARTITION_start_cloning_o_t,
    ) -> PK_ERROR_code_t;

    /// Stop cloning mode for a partition.
    pub fn PK_PARTITION_stop_cloning(
        partition: PK_PARTITION_t,
        options: *mut PK_PARTITION_stop_cloning_o_t,
    ) -> PK_ERROR_code_t;

    /// Check if a partition is a clone.
    pub fn PK_PARTITION_is_clone(
        partition: PK_PARTITION_t,
        original_pmark: PK_PMARK_t,
        clone_pmark: PK_PMARK_t,
        options: *mut PK_PARTITION_is_clone_o_t,
        is_clone: *mut PK_clone_state_t,
        n_entities: *mut c_int,
        entities: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Ask cloning state of a partition.
    pub fn PK_PARTITION_ask_cloning(
        partition: PK_PARTITION_t,
        options: *mut PK_PARTITION_ask_cloning_o_t,
        n_clone_records: *mut c_int,
        clone_records: *mut *mut PK_clone_record_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Partition misc queries
    // =========================================================================

    /// General partition ask (returns partition data).
    pub fn PK_PARTITION_ask(
        partition: PK_PARTITION_t,
        options: *mut PK_PARTITION_ask_o_t,
        results: *mut PK_PARTITION_ask_r_t,
    ) -> PK_ERROR_code_t;

    /// Reset attributes in a partition.
    pub fn PK_PARTITION_reset_attribs(
        partition: PK_PARTITION_t,
        n_attdefs: c_int,
        attdefs: *mut PK_ATTDEF_t,
        reset_cb: PK_ATTRIB_reset_cb_f_t,
        context: PK_POINTER_t,
        options: *mut PK_PARTITION_reset_attribs_o_t,
    ) -> PK_ERROR_code_t;

    /// Ask whether partition has facet geometry.
    pub fn PK_PARTITION_ask_facet_geom(
        partition: PK_PARTITION_t,
        has_facet_geom: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Ask whether partition contains lattices.
    pub fn PK_PARTITION_has_lattices(
        partition: PK_PARTITION_t,
        has_lattices: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Ask KI lists in partition.
    pub fn PK_PARTITION_ask_ki_lists(
        partition: PK_PARTITION_t,
        n_ki_lists: *mut c_int,
        ki_lists: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Roll-forward (_r_f) variants
    // =========================================================================

    /// Create partition (roll-forward variant).
    pub fn PK_PARTITION_create_r_f(
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// General partition ask (roll-forward variant).
    pub fn PK_PARTITION_ask_r_f(
        partition: PK_PARTITION_t,
        data: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Return all pmarks in partition (version 2, roll-forward variant).
    pub fn PK_PARTITION_ask_pmarks_2_r_f(
        results: *mut PK_PARTITION_ask_pmarks_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Set a guard on a partition (roll-forward variant).
    pub fn PK_PARTITION_set_guard_r_f(
        partition: PK_PARTITION_t,
        pmark: PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Roll partition to its guard pmark (roll-forward variant).
    pub fn PK_PARTITION_goto_guard_r_f(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Check if partition has lattices (roll-forward variant).
    pub fn PK_PARTITION_has_lattices_r_f(
        partition: PK_PARTITION_t,
        has_lattices: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Create session mark (roll-forward variant).
    pub fn PK_MARK_create_r_f(
        mark: *mut PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Delete session mark (roll-forward variant).
    pub fn PK_MARK_delete_r_f(
        mark: PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Free memory block.
    pub fn PK_MEMORY_block_f(
        block: *mut PK_MEMORY_block_t,
    ) -> PK_ERROR_code_t;
}
