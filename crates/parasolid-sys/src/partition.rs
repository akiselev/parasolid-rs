//! Partition, partition-mark (PMARK), session-mark (MARK), and delta bindings.
//!
//! Covers partition management, entity/partition queries, partition marks,
//! session marks, deltas, and rollback functions from Parasolid chapters 94-95.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_int, c_void};

use crate::*;

// =============================================================================
// Partition type enum
// =============================================================================

/// Partition type: standard (full delta history) or light (no backward delta
/// at first non-initial pmark).
pub type PK_partition_type_t = c_int;

/// Standard partition — full delta history (default).
pub const PK_partition_type_standard_c: PK_partition_type_t = 0;

/// Light partition — no backward delta at first non-initial pmark.
pub const PK_partition_type_light_c: PK_partition_type_t = 1;

// =============================================================================
// PK_PARTITION_copy — copy_deltas enum
// =============================================================================

/// Controls which deltas are copied during `PK_PARTITION_copy`.
pub type PK_partition_copy_deltas_t = c_int;

/// Do not copy any deltas.
pub const PK_partition_copy_deltas_no_c: PK_partition_copy_deltas_t = 0;

/// Copy all deltas in the partition.
pub const PK_partition_copy_deltas_all_c: PK_partition_copy_deltas_t = 1;

/// Copy only deltas on the main line (initial pmark to current pmark).
pub const PK_partition_copy_deltas_main_c: PK_partition_copy_deltas_t = 2;

/// Create delta for current pmark only.
pub const PK_partition_copy_deltas_current_c: PK_partition_copy_deltas_t = 3;

// =============================================================================
// PK_PARTITION_delete options
// =============================================================================

/// Options for `PK_PARTITION_delete`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARTITION_delete_o_t {
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

/// Delta callback: open/create delta file for output (associated with pmark).
pub type PK_DELTA_open_for_write_f_t = Option<
    unsafe extern "C" fn(
        pmark: PK_PMARK_t,
        strid: *mut c_int,
        ifail: *mut c_int,
    ),
>;

/// Delta callback: open delta for input.
pub type PK_DELTA_open_for_read_f_t = Option<
    unsafe extern "C" fn(
        pmark: PK_PMARK_t,
        strid: *mut c_int,
        ifail: *mut c_int,
    ),
>;

/// Delta callback: close delta.
pub type PK_DELTA_close_f_t = Option<
    unsafe extern "C" fn(
        strid: c_int,
        ifail: *mut c_int,
    ),
>;

/// Delta callback: write n_bytes to delta.
pub type PK_DELTA_write_f_t = Option<
    unsafe extern "C" fn(
        strid: c_int,
        n_bytes: c_int,
        buffer: *const c_void,
        ifail: *mut c_int,
    ),
>;

/// Delta callback: read n_bytes from delta.
pub type PK_DELTA_read_f_t = Option<
    unsafe extern "C" fn(
        strid: c_int,
        max_bytes: c_int,
        buffer: *mut c_void,
        actual_bytes: *mut c_int,
        ifail: *mut c_int,
    ),
>;

/// Delta callback: delete delta.
pub type PK_DELTA_delete_f_t = Option<
    unsafe extern "C" fn(
        pmark: PK_PMARK_t,
        ifail: *mut c_int,
    ),
>;

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
pub const PK_REPORT_1_no_roll_diff_c: c_int = 1;

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

    /// Create a new partition in the session.
    pub fn PK_PARTITION_create(
        partition: *mut PK_PARTITION_t,
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

    /// Delete partitions.
    pub fn PK_PARTITION_delete(
        n_partitions: c_int,
        partitions: *const PK_PARTITION_t,
        options: *const PK_PARTITION_delete_o_t,
    ) -> PK_ERROR_code_t;

    /// Merge two or more partitions together.
    pub fn PK_PARTITION_merge(
        n_partitions: c_int,
        partitions: *const PK_PARTITION_t,
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
        pmarks: *const PK_PMARK_t,
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
        result: *mut PK_PMARK_ask_entities_r_t,
    ) -> PK_ERROR_code_t;

    /// Roll partition to specified pmark.
    pub fn PK_PMARK_goto(
        pmark: PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Roll partition to specified pmark (version 2, with options and return data).
    pub fn PK_PMARK_goto_2(
        pmark: PK_PMARK_t,
        options: *const PK_PMARK_goto_2_o_t,
        result: *mut PK_PMARK_goto_2_r_t,
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
        options: *const PK_PARTITION_make_pmark_2_o_t,
        pmark: *mut PK_PMARK_t,
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

    /// Advance most recent pmark to current state.
    pub fn PK_PARTITION_advance_pmark(
        partition: PK_PARTITION_t,
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

    /// Ask mark state (superseded by `PK_SESSION_ask_mark`).
    pub fn PK_MARK_ask_state(
        mark: PK_MARK_t,
        is_current: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return whether mark rollback is active.
    pub fn PK_MARK_is_on(
        is_on: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the forward-looking mark following the given mark.
    pub fn PK_MARK_ask_forward(
        mark: PK_MARK_t,
        forward: *mut PK_MARK_t,
    ) -> PK_ERROR_code_t;

    /// Return the frustrum data associated with a mark.
    pub fn PK_MARK_ask_frustrum(
        mark: PK_MARK_t,
        frustrum: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Start mark-based rollback (legacy).
    pub fn PK_MARK_start() -> PK_ERROR_code_t;

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
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Receive partition (binary format).
    pub fn PK_PARTITION_receive_b(
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Receive partition (Unicode format).
    pub fn PK_PARTITION_receive_u(
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Transmit (export) a partition to Frustrum.
    pub fn PK_PARTITION_transmit(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Transmit partition (binary format).
    pub fn PK_PARTITION_transmit_b(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Transmit partition (Unicode format).
    pub fn PK_PARTITION_transmit_u(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Transmit a delta between pmarks.
    pub fn PK_PARTITION_transmit_delta(
        partition: PK_PARTITION_t,
        pmark: PK_PMARK_t,
    ) -> PK_ERROR_code_t;

    /// Receive deltas for a partition.
    pub fn PK_PARTITION_receive_deltas(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Receive deltas for a partition (version 2).
    pub fn PK_PARTITION_receive_deltas_2(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Receive partition version information.
    pub fn PK_PARTITION_receive_version(
        version: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Receive partition version (binary format).
    pub fn PK_PARTITION_receive_version_b(
        version: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Receive partition version (Unicode format).
    pub fn PK_PARTITION_receive_version_u(
        version: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Receive mesh data into partition.
    pub fn PK_PARTITION_receive_meshes(
        partition: PK_PARTITION_t,
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
    ) -> PK_ERROR_code_t;

    /// Stop cloning mode for a partition.
    pub fn PK_PARTITION_stop_cloning(
        partition: PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Check if a partition is a clone.
    pub fn PK_PARTITION_is_clone(
        partition: PK_PARTITION_t,
        is_clone: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Ask cloning state of a partition.
    pub fn PK_PARTITION_ask_cloning(
        partition: PK_PARTITION_t,
        is_cloning: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Partition misc queries
    // =========================================================================

    /// General partition ask (returns partition data).
    pub fn PK_PARTITION_ask(
        partition: PK_PARTITION_t,
        data: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Reset attributes in a partition.
    pub fn PK_PARTITION_reset_attribs(
        partition: PK_PARTITION_t,
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
        partition: PK_PARTITION_t,
        n_pmarks: *mut c_int,
        pmarks: *mut *mut PK_PMARK_t,
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
}
