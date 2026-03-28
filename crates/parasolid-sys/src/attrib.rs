//! Attributes, attribute definitions, application items, groups, and bulletin board.
//!
//! Covers PK_ATTRIB_*, PK_ATTDEF_*, PK_APPITEM_*, PK_GROUP_*, PK_BB_*,
//! and related PK_ENTITY_*/PK_PART_* attribute/group functions (chapters 91-97).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::*;
use std::os::raw::{c_char, c_double, c_int, c_void};

// =============================================================================
// Pointer type for application data
// =============================================================================

/// Opaque application pointer type used in attribute pointer fields and appitems.
pub type PK_POINTER_t = *mut c_void;

// =============================================================================
// Attribute field type constants
// =============================================================================

pub type PK_ATTRIB_field_t = c_int;

/// Real (double) field.
pub const PK_ATTRIB_field_real_c: PK_ATTRIB_field_t = 0;
/// Integer field.
pub const PK_ATTRIB_field_integer_c: PK_ATTRIB_field_t = 1;
/// ASCII string field.
pub const PK_ATTRIB_field_string_c: PK_ATTRIB_field_t = 2;
/// Unicode string field.
pub const PK_ATTRIB_field_ustring_c: PK_ATTRIB_field_t = 3;
/// Vector (displacement) field.
pub const PK_ATTRIB_field_vector_c: PK_ATTRIB_field_t = 4;
/// Coordinate (position) field.
pub const PK_ATTRIB_field_coordinate_c: PK_ATTRIB_field_t = 5;
/// Direction (unit length) field.
pub const PK_ATTRIB_field_direction_c: PK_ATTRIB_field_t = 6;
/// Axis (coordinate + direction) field.
pub const PK_ATTRIB_field_axis_c: PK_ATTRIB_field_t = 7;
/// Pointer field (not transmitted).
pub const PK_ATTRIB_field_pointer_c: PK_ATTRIB_field_t = 8;

// =============================================================================
// Attribute definition class constants
// =============================================================================

pub type PK_ATTDEF_class_t = c_int;

/// Class 1 -- independent of entity position and size.
pub const PK_ATTDEF_class_01_c: PK_ATTDEF_class_t = 1;
/// Class 2.
pub const PK_ATTDEF_class_02_c: PK_ATTDEF_class_t = 2;
/// Class 3.
pub const PK_ATTDEF_class_03_c: PK_ATTDEF_class_t = 3;
/// Class 4.
pub const PK_ATTDEF_class_04_c: PK_ATTDEF_class_t = 4;
/// Class 5.
pub const PK_ATTDEF_class_05_c: PK_ATTDEF_class_t = 5;
/// Class 6 -- like class 1 but supports multiple values.
pub const PK_ATTDEF_class_06_c: PK_ATTDEF_class_t = 6;
/// Class 7.
pub const PK_ATTDEF_class_07_c: PK_ATTDEF_class_t = 7;

// =============================================================================
// Attribute definition callback type constants
// =============================================================================

pub type PK_ATTDEF_callback_t = c_int;

/// Normal callback (replaces Parasolid processing).
pub const PK_ATTDEF_callback_normal_c: PK_ATTDEF_callback_t = 0;
/// Read-only callback (supplements Parasolid processing).
pub const PK_ATTDEF_callback_read_only_c: PK_ATTDEF_callback_t = 1;

// =============================================================================
// No-roll attribute constants
// =============================================================================

pub type PK_ATTRIB_no_roll_t = c_int;

/// Report field value changes due to rollback of no-roll attributes.
pub const PK_ATTRIB_no_roll_diff_report_c: PK_ATTRIB_no_roll_t = 1;

// =============================================================================
// Group option constants
// =============================================================================

pub type PK_GROUP_dependants_t = c_int;

/// Do not delete member groups on group deletion (default).
pub const PK_GROUP_dependants_keep_c: PK_GROUP_dependants_t = 0;
/// Consider deleting member groups not in other groups.
pub const PK_GROUP_dependants_delete_c: PK_GROUP_dependants_t = 1;

pub type PK_GROUP_membership_t = c_int;

/// Merged entity always in group (default).
pub const PK_GROUP_membership_inclusive_c: PK_GROUP_membership_t = 0;
/// Merged entity in group only if both originals were.
pub const PK_GROUP_membership_exclusive_c: PK_GROUP_membership_t = 1;

pub type PK_GROUP_merge_t = c_int;

/// Do not merge identical groups (default).
pub const PK_GROUP_merge_no_c: PK_GROUP_merge_t = 0;
/// Merge identical groups.
pub const PK_GROUP_merge_identical_c: PK_GROUP_merge_t = 1;

pub type PK_GROUP_split_t = c_int;

/// Empty group stays in original part on body split (default).
pub const PK_GROUP_split_never_c: PK_GROUP_split_t = 0;
/// Empty group stays + copy created in split part.
pub const PK_GROUP_split_always_c: PK_GROUP_split_t = 1;

// System attribute group split/merge constants (from SDL/TYSA_GROUP_CONTROL).

pub type PK_GROUP_split_empty_t = c_int;

/// Group remains in original part (default).
pub const PK_GROUP_split_empty_no_c: PK_GROUP_split_empty_t = 0;
/// Group remains in original and is copied to split-off part.
pub const PK_GROUP_split_empty_copy_c: PK_GROUP_split_empty_t = 1;

pub type PK_GROUP_merge_empty_t = c_int;

/// Empty groups in both parts appear in merged part (default).
pub const PK_GROUP_merge_empty_no_c: PK_GROUP_merge_empty_t = 0;
/// Identical empty groups (one per part) merge into one.
pub const PK_GROUP_merge_empty_yes_c: PK_GROUP_merge_empty_t = 1;

// =============================================================================
// Bulletin board constants
// =============================================================================

pub type PK_BB_status_t = c_int;

/// Switch BB on to record entities.
pub const PK_BB_status_on_c: PK_BB_status_t = 1;
/// Switch BB off and clear contents.
pub const PK_BB_status_off_c: PK_BB_status_t = 0;
/// Switch BB on to record entities and user fields.
pub const PK_BB_status_user_field_c: PK_BB_status_t = 2;

pub type PK_BB_event_t = c_int;

/// Creation event.
pub const PK_BB_event_create_c: PK_BB_event_t = 0;
/// Copy event.
pub const PK_BB_event_copy_c: PK_BB_event_t = 1;

// =============================================================================
// Attribute definition standard form structures
// =============================================================================

/// Standard form of an attribute definition (numbered fields).
///
/// Returned by `PK_ATTDEF_ask`. Fields describe the definition's name,
/// class, owning entity classes, and field types.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_ATTDEF_sf_t {
    /// Name string identifying this attribute definition.
    pub name: *const c_char,
    /// Attribute class (PK_ATTDEF_class_01_c .. PK_ATTDEF_class_07_c).
    pub attdef_class: PK_ATTDEF_class_t,
    /// Number of entity classes that may own attributes of this definition.
    pub n_owner_classes: c_int,
    /// Array of entity class tokens that may own this attribute.
    pub owner_classes: *const PK_CLASS_t,
    /// Number of fields in the attribute.
    pub n_fields: c_int,
    /// Array of field type tokens.
    pub field_types: *const PK_ATTRIB_field_t,
}

/// Standard form of an attribute definition (named fields, version 2).
///
/// Returned by `PK_ATTDEF_ask_2`. Like `PK_ATTDEF_sf_t` but each field
/// also has a name string.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_ATTDEF_sf_2_t {
    /// Name string identifying this attribute definition.
    pub name: *const c_char,
    /// Attribute class.
    pub attdef_class: PK_ATTDEF_class_t,
    /// Number of entity classes that may own attributes of this definition.
    pub n_owner_classes: c_int,
    /// Array of entity class tokens.
    pub owner_classes: *const PK_CLASS_t,
    /// Number of fields.
    pub n_fields: c_int,
    /// Array of field type tokens.
    pub field_types: *const PK_ATTRIB_field_t,
    /// Array of field name strings.
    pub field_names: *const *const c_char,
}

/// Union of field value arrays used in attribute callbacks.
#[repr(C)]
#[derive(Clone, Copy)]
pub union PK_ATTRIB_field_values_t {
    pub reals: *const c_double,
    pub ints: *const c_int,
    pub string: *const c_char,
    pub vectors: *const c_double,
    pub pointers: *const PK_POINTER_t,
}

// =============================================================================
// Callback types for attribute operations
// =============================================================================

/// Callback for `PK_PART_ask_attribs_cb` -- called per attribute.
/// Returns PK_LOGICAL_true to continue, PK_LOGICAL_false to stop.
pub type PK_PART_ask_attribs_cb_fn_t = Option<
    unsafe extern "C" fn(
        attdef_sf: *const PK_ATTDEF_sf_2_t,
        field_lengths: *const c_int,
        field_values: *const PK_ATTRIB_field_values_t,
        context: PK_POINTER_t,
    ) -> PK_LOGICAL_t,
>;

/// Callback for rollback attribute deletion (del_attrib_cb in PK_PMARK_goto_2).
pub type PK_del_attrib_cb_fn_t = Option<
    unsafe extern "C" fn(
        attrib: PK_ATTRIB_t,
        attdef: PK_ATTDEF_t,
        context: PK_POINTER_t,
    ),
>;

/// Attribute filter callback function type.
pub type PK_ATTRIB_filter_f_t = Option<unsafe extern "C" fn(PK_ATTRIB_t, PK_POINTER_t) -> PK_LOGICAL_t>;

// =============================================================================
// Bulletin board setup structure
// =============================================================================

/// Setup structure for the bulletin board.
///
/// Each field is a count+array pair specifying which entity classes to track
/// for that event type.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BB_sf_t {
    pub n_create: c_int,
    pub create: *const PK_CLASS_t,
    pub n_copy: c_int,
    pub copy: *const PK_CLASS_t,
    pub n_deleet: c_int,
    pub deleet: *const PK_CLASS_t,
    pub n_split: c_int,
    pub split: *const PK_CLASS_t,
    pub n_merge: c_int,
    pub merge: *const PK_CLASS_t,
    pub n_transform: c_int,
    pub transform: *const PK_CLASS_t,
    pub n_transfer: c_int,
    pub transfer: *const PK_CLASS_t,
    pub n_change: c_int,
    pub change: *const PK_CLASS_t,
    pub n_change_to_attribute: c_int,
    pub change_to_attribute: *const PK_CLASS_t,
}

// =============================================================================
// Extern "C" function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {

    // =========================================================================
    // PK_APPITEM — application items (Ch. 91)
    // =========================================================================

    /// Create appitems referring to application data indicated by given pointers.
    pub fn PK_APPITEM_create(
        n_pointers: c_int,
        pointers: *const PK_POINTER_t,
        n_appitems: *mut c_int,
        appitems: *mut *mut PK_APPITEM_t,
    ) -> PK_ERROR_code_t;

    /// Delete the given appitems.
    pub fn PK_APPITEM_delete(
        n_appitems: c_int,
        appitems: *const PK_APPITEM_t,
    ) -> PK_ERROR_code_t;

    /// Return the pointers for the given appitems.
    pub fn PK_APPITEM_ask(
        n_appitems: c_int,
        appitems: *const PK_APPITEM_t,
        n_pointers: *mut c_int,
        pointers: *mut *mut PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    /// Overwrite the pointers in supplied appitems with replacement pointers.
    pub fn PK_APPITEM_reset_pointers(
        n_appitems: c_int,
        appitems: *const PK_APPITEM_t,
        n_pointers: c_int,
        pointers: *const PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    /// Test whether the given argument is an appitem.
    pub fn PK_APPITEM_is(
        entity: PK_ENTITY_t,
        is_appitem: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // PK_ATTDEF — attribute definitions (Ch. 92)
    // =========================================================================

    /// Create an attribute definition (numbered fields).
    pub fn PK_ATTDEF_create(
        name: *const c_char,
        attdef_class: PK_ATTDEF_class_t,
        n_owner_classes: c_int,
        owner_classes: *const PK_CLASS_t,
        n_fields: c_int,
        field_types: *const PK_ATTRIB_field_t,
        attdef: *mut PK_ATTDEF_t,
    ) -> PK_ERROR_code_t;

    /// Create an attribute definition with named fields.
    pub fn PK_ATTDEF_create_2(
        name: *const c_char,
        attdef_class: PK_ATTDEF_class_t,
        n_owner_classes: c_int,
        owner_classes: *const PK_CLASS_t,
        n_fields: c_int,
        field_types: *const PK_ATTRIB_field_t,
        field_names: *const *const c_char,
        attdef: *mut PK_ATTDEF_t,
    ) -> PK_ERROR_code_t;

    /// Return standard form for attribute definition (non-named fields).
    pub fn PK_ATTDEF_ask(
        attdef: PK_ATTDEF_t,
        sf: *mut PK_ATTDEF_sf_t,
    ) -> PK_ERROR_code_t;

    /// Return standard form for attribute definition (named fields).
    pub fn PK_ATTDEF_ask_2(
        attdef: PK_ATTDEF_t,
        sf: *mut PK_ATTDEF_sf_2_t,
    ) -> PK_ERROR_code_t;

    /// Map from an attribute name to a definition.
    pub fn PK_ATTDEF_find(
        name: *const c_char,
        attdef: *mut PK_ATTDEF_t,
    ) -> PK_ERROR_code_t;

    /// Ask whether the given attdef is a group closing attribute definition.
    pub fn PK_ATTDEF_is_group_closing(
        attdef: PK_ATTDEF_t,
        is_closing: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Sets the given attdef to be group closing.
    pub fn PK_ATTDEF_set_group_closing(
        attdef: PK_ATTDEF_t,
    ) -> PK_ERROR_code_t;

    /// Register pointers to callback functions for attribute processing.
    pub fn PK_ATTDEF_register_cb(
        attdef: PK_ATTDEF_t,
        callback_type: PK_ATTDEF_callback_t,
        callback: *const c_void,
    ) -> PK_ERROR_code_t;

    /// Register callbacks for attribute processing (alternate form).
    pub fn PK_ATTDEF_register_callbacks(
        attdef: PK_ATTDEF_t,
        callback_type: PK_ATTDEF_callback_t,
        callback: *const c_void,
    ) -> PK_ERROR_code_t;

    /// Return registered callback pointers.
    pub fn PK_ATTDEF_ask_callbacks(
        attdef: PK_ATTDEF_t,
        callback_type: *mut PK_ATTDEF_callback_t,
        callback: *mut *const c_void,
    ) -> PK_ERROR_code_t;

    /// Set callback on/off flags for attribute definition events.
    pub fn PK_ATTDEF_set_callback_flags(
        attdef: PK_ATTDEF_t,
        flags: c_int,
    ) -> PK_ERROR_code_t;

    /// Return callback on/off flags.
    pub fn PK_ATTDEF_ask_callback_flags(
        attdef: PK_ATTDEF_t,
        flags: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Set context data for an attribute definition.
    pub fn PK_ATTDEF_set_contexts(
        attdef: PK_ATTDEF_t,
        context: PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    /// Return context data for an attribute definition.
    pub fn PK_ATTDEF_ask_contexts(
        attdef: PK_ATTDEF_t,
        context: *mut PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    /// Ask whether a given entity may own an attribute of a given definition.
    pub fn PK_ENTITY_may_own_attdef(
        entity: PK_ENTITY_t,
        attdef: PK_ATTDEF_t,
        may_own: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // PK_ATTRIB — attributes (Ch. 93)
    // =========================================================================

    // --- Creating and querying ---

    /// Create a new attribute attached to entity with given definition (no data).
    pub fn PK_ATTRIB_create_empty(
        entity: PK_ENTITY_t,
        attdef: PK_ATTDEF_t,
        attrib: *mut PK_ATTRIB_t,
    ) -> PK_ERROR_code_t;

    /// Return the entity owning an attribute.
    pub fn PK_ATTRIB_ask_owner(
        attrib: PK_ATTRIB_t,
        owner: *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Return the attribute definition of an attribute.
    pub fn PK_ATTRIB_ask_attdef(
        attrib: PK_ATTRIB_t,
        attdef: *mut PK_ATTDEF_t,
    ) -> PK_ERROR_code_t;

    // --- Numbered field write ---

    /// Write axes data to numbered field.
    pub fn PK_ATTRIB_set_axes(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_axes: c_int,
        axes: *const c_double,
    ) -> PK_ERROR_code_t;

    /// Write doubles to numbered field.
    pub fn PK_ATTRIB_set_doubles(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_doubles: c_int,
        doubles: *const c_double,
    ) -> PK_ERROR_code_t;

    /// Write ints to numbered field.
    pub fn PK_ATTRIB_set_ints(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_ints: c_int,
        ints: *const c_int,
    ) -> PK_ERROR_code_t;

    /// Write string to numbered field.
    pub fn PK_ATTRIB_set_string(
        attrib: PK_ATTRIB_t,
        field: c_int,
        string: *const c_char,
    ) -> PK_ERROR_code_t;

    /// Write Unicode string to numbered field.
    pub fn PK_ATTRIB_set_ustring(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_chars: c_int,
        ustring: *const c_int,
    ) -> PK_ERROR_code_t;

    /// Write vectors to numbered field.
    pub fn PK_ATTRIB_set_vectors(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_vectors: c_int,
        vectors: *const c_double,
    ) -> PK_ERROR_code_t;

    /// Write pointers to numbered field.
    pub fn PK_ATTRIB_set_pointers(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_pointers: c_int,
        pointers: *const PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    // --- Numbered field read (whole field) ---

    /// Read all axes from numbered field.
    pub fn PK_ATTRIB_ask_axes(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_axes: *mut c_int,
        axes: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Read all doubles from numbered field.
    pub fn PK_ATTRIB_ask_doubles(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_doubles: *mut c_int,
        doubles: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Read all ints from numbered field.
    pub fn PK_ATTRIB_ask_ints(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_ints: *mut c_int,
        ints: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Read string from numbered field.
    pub fn PK_ATTRIB_ask_string(
        attrib: PK_ATTRIB_t,
        field: c_int,
        string: *mut *mut c_char,
    ) -> PK_ERROR_code_t;

    /// Read Unicode string from numbered field.
    pub fn PK_ATTRIB_ask_ustring(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_chars: *mut c_int,
        ustring: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Read all vectors from numbered field.
    pub fn PK_ATTRIB_ask_vectors(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_vectors: *mut c_int,
        vectors: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Read all pointers from numbered field.
    pub fn PK_ATTRIB_ask_pointers(
        attrib: PK_ATTRIB_t,
        field: c_int,
        n_pointers: *mut c_int,
        pointers: *mut *mut PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    // --- Numbered field read (nth item) ---

    /// Read nth axis from field.
    pub fn PK_ATTRIB_ask_nth_axis(
        attrib: PK_ATTRIB_t,
        field: c_int,
        index: c_int,
        axis: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Read nth double from field.
    pub fn PK_ATTRIB_ask_nth_double(
        attrib: PK_ATTRIB_t,
        field: c_int,
        index: c_int,
        value: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Read nth int from field.
    pub fn PK_ATTRIB_ask_nth_int(
        attrib: PK_ATTRIB_t,
        field: c_int,
        index: c_int,
        value: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Read nth vector from field.
    pub fn PK_ATTRIB_ask_nth_vector(
        attrib: PK_ATTRIB_t,
        field: c_int,
        index: c_int,
        vector: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Read nth pointer from field.
    pub fn PK_ATTRIB_ask_nth_pointer(
        attrib: PK_ATTRIB_t,
        field: c_int,
        index: c_int,
        pointer: *mut PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    // --- Named field write ---

    /// Write axes to named field.
    pub fn PK_ATTRIB_set_named_axes(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_axes: c_int,
        axes: *const c_double,
    ) -> PK_ERROR_code_t;

    /// Write doubles to named field.
    pub fn PK_ATTRIB_set_named_doubles(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_doubles: c_int,
        doubles: *const c_double,
    ) -> PK_ERROR_code_t;

    /// Write ints to named field.
    pub fn PK_ATTRIB_set_named_ints(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_ints: c_int,
        ints: *const c_int,
    ) -> PK_ERROR_code_t;

    /// Write string to named field.
    pub fn PK_ATTRIB_set_named_string(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        string: *const c_char,
    ) -> PK_ERROR_code_t;

    /// Write Unicode string to named field.
    pub fn PK_ATTRIB_set_named_ustring(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_chars: c_int,
        ustring: *const c_int,
    ) -> PK_ERROR_code_t;

    /// Write vectors to named field.
    pub fn PK_ATTRIB_set_named_vectors(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_vectors: c_int,
        vectors: *const c_double,
    ) -> PK_ERROR_code_t;

    /// Write pointers to named field.
    pub fn PK_ATTRIB_set_named_pointers(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_pointers: c_int,
        pointers: *const PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    // --- Named field read ---

    /// Read all axes from named field.
    pub fn PK_ATTRIB_ask_named_axes(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_axes: *mut c_int,
        axes: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Read all doubles from named field.
    pub fn PK_ATTRIB_ask_named_doubles(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_doubles: *mut c_int,
        doubles: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Read all ints from named field.
    pub fn PK_ATTRIB_ask_named_ints(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_ints: *mut c_int,
        ints: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Read string from named field.
    pub fn PK_ATTRIB_ask_named_string(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        string: *mut *mut c_char,
    ) -> PK_ERROR_code_t;

    /// Read Unicode string from named field.
    pub fn PK_ATTRIB_ask_named_ustring(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_chars: *mut c_int,
        ustring: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Read all vectors from named field.
    pub fn PK_ATTRIB_ask_named_vectors(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_vectors: *mut c_int,
        vectors: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Read all pointers from named field.
    pub fn PK_ATTRIB_ask_named_pointers(
        attrib: PK_ATTRIB_t,
        field_name: *const c_char,
        n_pointers: *mut c_int,
        pointers: *mut *mut PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    // --- No-roll attributes ---

    /// Set attribute as no-roll (field values survive rollback).
    pub fn PK_ATTRIB_set_no_roll(
        attrib: PK_ATTRIB_t,
        no_roll: PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Check if attribute is a no-roll attribute.
    pub fn PK_ATTRIB_ask_no_roll(
        attrib: PK_ATTRIB_t,
        no_roll: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // --- Deleting attributes ---

    /// Delete attributes of specified attdefs from all entities (or subclass) in a part.
    pub fn PK_PART_delete_attribs(
        part: PK_PART_t,
        n_attdefs: c_int,
        attdefs: *const PK_ATTDEF_t,
        eclass: PK_CLASS_t,
    ) -> PK_ERROR_code_t;

    // --- Querying attributes ---

    /// Retrieve all attributes on an entity.
    pub fn PK_ENTITY_ask_attribs(
        entity: PK_ENTITY_t,
        n_attribs: *mut c_int,
        attribs: *mut *mut PK_ATTRIB_t,
    ) -> PK_ERROR_code_t;

    /// Retrieve a single (first) attribute on an entity with given definition.
    pub fn PK_ENTITY_ask_first_attrib(
        entity: PK_ENTITY_t,
        attdef: PK_ATTDEF_t,
        attrib: *mut PK_ATTRIB_t,
    ) -> PK_ERROR_code_t;

    /// Get all attributes anywhere in the part.
    pub fn PK_PART_ask_all_attribs(
        part: PK_PART_t,
        n_attribs: *mut c_int,
        attribs: *mut *mut PK_ATTRIB_t,
    ) -> PK_ERROR_code_t;

    /// Return selected attributes via callback function.
    pub fn PK_PART_ask_attribs_cb(
        part: PK_PART_t,
        n_attdefs: c_int,
        attdefs: *const PK_ATTDEF_t,
        callback: PK_PART_ask_attribs_cb_fn_t,
        context: PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // PK_GROUP — groups (Ch. 96)
    // =========================================================================

    /// Create group from array of entities (with options).
    pub fn PK_GROUP_create_from_entities_2(
        part: PK_PART_t,
        entity_class: PK_CLASS_t,
        n_entities: c_int,
        entities: *const PK_ENTITY_t,
        group: *mut PK_GROUP_t,
    ) -> PK_ERROR_code_t;

    /// Create group from array of entities (legacy, deprecated).
    pub fn PK_GROUP_create_from_entities(
        part: PK_PART_t,
        entity_class: PK_CLASS_t,
        n_entities: c_int,
        entities: *const PK_ENTITY_t,
        group: *mut PK_GROUP_t,
    ) -> PK_ERROR_code_t;

    /// Add more entities to a group.
    pub fn PK_GROUP_add_entities(
        group: PK_GROUP_t,
        n_entities: c_int,
        entities: *const PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Add entities, purging duplicates.
    pub fn PK_GROUP_merge_entities(
        group: PK_GROUP_t,
        n_entities: c_int,
        entities: *const PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Remove entities from a group.
    pub fn PK_GROUP_remove_entities(
        group: PK_GROUP_t,
        n_entities: c_int,
        entities: *const PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Return entities in the group.
    pub fn PK_GROUP_ask_entities(
        group: PK_GROUP_t,
        n_entities: *mut c_int,
        entities: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Search a group for a specific element.
    pub fn PK_GROUP_contains_entity(
        group: PK_GROUP_t,
        entity: PK_ENTITY_t,
        contains: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// What class are the grouped entities?
    pub fn PK_GROUP_ask_entity_class(
        group: PK_GROUP_t,
        entity_class: *mut PK_CLASS_t,
    ) -> PK_ERROR_code_t;

    /// Set integer label for entity in group.
    pub fn PK_GROUP_set_entity_label(
        group: PK_GROUP_t,
        entity: PK_ENTITY_t,
        label: c_int,
    ) -> PK_ERROR_code_t;

    /// Return integer label for entity in group.
    pub fn PK_GROUP_ask_entity_label(
        group: PK_GROUP_t,
        entity: PK_ENTITY_t,
        label: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Return entities in group, optionally filtered by label.
    pub fn PK_GROUP_find_entities(
        group: PK_GROUP_t,
        label: c_int,
        n_entities: *mut c_int,
        entities: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Return the controls of the given group.
    pub fn PK_GROUP_ask_controls(
        group: PK_GROUP_t,
        controls: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Ask whether group is a closed group.
    pub fn PK_GROUP_ask_closure(
        group: PK_GROUP_t,
        is_closed: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Find groups that an entity is member of (legacy).
    pub fn PK_ENTITY_ask_owning_groups(
        entity: PK_ENTITY_t,
        n_groups: *mut c_int,
        groups: *mut *mut PK_GROUP_t,
    ) -> PK_ERROR_code_t;

    /// Find groups in a part (legacy).
    pub fn PK_PART_ask_groups(
        part: PK_PART_t,
        n_groups: *mut c_int,
        groups: *mut *mut PK_GROUP_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // PK_BB — bulletin board (Ch. 97)
    // =========================================================================

    /// Set up the bulletin board: specify which entity/event combinations to record.
    pub fn PK_BB_create(
        sf: *const PK_BB_sf_t,
        bb: *mut PK_BB_t,
    ) -> PK_ERROR_code_t;

    /// Switch bulletin board on/off.
    pub fn PK_BB_set_status(
        bb: PK_BB_t,
        status: PK_BB_status_t,
    ) -> PK_ERROR_code_t;

    /// Query the current setup of the bulletin board.
    pub fn PK_BB_ask(
        bb: PK_BB_t,
        sf: *mut PK_BB_sf_t,
    ) -> PK_ERROR_code_t;

    /// Query whether the BB is currently recording.
    pub fn PK_BB_ask_status(
        bb: PK_BB_t,
        status: *mut PK_BB_status_t,
    ) -> PK_ERROR_code_t;

    /// Determine whether an entity is a bulletin board.
    pub fn PK_BB_is(
        entity: PK_ENTITY_t,
        is_bb: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Query whether the bulletin board is empty.
    pub fn PK_BB_is_empty(
        bb: PK_BB_t,
        is_empty: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Output events since last emptied; optionally empty the BB.
    ///
    /// Set `empty` to `PK_LOGICAL_true` to empty the bulletin board after reading.
    pub fn PK_BB_output_events(
        bb: PK_BB_t,
        empty: PK_LOGICAL_t,
        n_events: *mut c_int,
        events: *mut *mut c_int,
        entities: *mut *mut PK_ENTITY_t,
        event_types: *mut *mut PK_BB_event_t,
        classes: *mut *mut PK_CLASS_t,
    ) -> PK_ERROR_code_t;

    /// Ensure no duplicate/invalid identifiers.
    pub fn PK_PART_rectify_identifiers(
        part: PK_PART_t,
        n_entities: *mut c_int,
        entities: *mut *mut PK_ENTITY_t,
        old_idents: *mut *mut c_int,
        new_idents: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Find entities with matching attribute fields.
    pub fn PK_PART_ask_attrib_owners(
        part: PK_PART_t,
        attdef: PK_ATTDEF_t,
        n_fields: c_int,
        fields: *const c_int,
        indices: *const c_int,
        values: *const c_int,
        filter: PK_ATTRIB_filter_f_t,
        context: PK_POINTER_t,
        n_entities: *mut c_int,
        entities: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Find attributes matching field values.
    pub fn PK_PART_ask_attribs_filter(
        part: PK_PART_t,
        attdef: PK_ATTDEF_t,
        n_fields: c_int,
        fields: *const c_int,
        indices: *const c_int,
        values: *const c_int,
        filter: PK_ATTRIB_filter_f_t,
        context: PK_POINTER_t,
        n_attribs: *mut c_int,
        attribs: *mut *mut PK_ATTRIB_t,
    ) -> PK_ERROR_code_t;

    /// Construction lattices on a part.
    pub fn PK_PART_ask_con_lattices(
        part: PK_PART_t,
        n_con_lattices: *mut c_int,
        con_lattices: *mut *mut PK_LATTICE_t,
    ) -> PK_ERROR_code_t;
}
