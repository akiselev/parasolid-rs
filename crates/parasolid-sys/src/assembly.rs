#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! FFI bindings for Parasolid assembly and instance functions
//! (`PK_ASSEMBLY_*`, `PK_INSTANCE_*`).
//!
//! Assemblies are collections of instances that reference parts (bodies or
//! sub-assemblies). Each instance carries a rigid-motion transform that
//! positions its part in the owning assembly's coordinate frame.

use std::os::raw::c_int;

use crate::*;

// =============================================================================
// Extern declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // ---- Assembly creation & modification -----------------------------------

    /// Create an empty assembly (no instances).
    pub fn PK_ASSEMBLY_create_empty(
        assembly: *mut PK_ASSEMBLY_t,
    ) -> PK_ERROR_code_t;

    /// Transform an assembly by composing a transformation onto all of its
    /// instances' transforms.
    pub fn PK_ASSEMBLY_transform(
        assembly: PK_ASSEMBLY_t,
        transf: PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Flatten an assembly hierarchy into a single-level assembly where every
    /// instance directly references a body (no sub-assemblies).
    pub fn PK_ASSEMBLY_make_level_assembly(
        assembly: PK_ASSEMBLY_t,
        level_assembly: *mut PK_ASSEMBLY_t,
    ) -> PK_ERROR_code_t;

    /// Check an assembly for validity.
    pub fn PK_ASSEMBLY_check(
        assembly: PK_ASSEMBLY_t,
        options: *mut PK_ASSEMBLY_check_o_t,
        results: *mut PK_ASSEMBLY_check_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Assembly navigation ------------------------------------------------

    /// Return the parts directly instanced by this assembly.
    /// Parts may appear more than once if instanced multiple times.
    pub fn PK_ASSEMBLY_ask_parts(
        assembly: PK_ASSEMBLY_t,
        n_parts: *mut c_int,
        parts: *mut *mut PK_PART_t,
    ) -> PK_ERROR_code_t;

    /// Return the instances directly owned by this assembly.
    pub fn PK_ASSEMBLY_ask_instances(
        assembly: PK_ASSEMBLY_t,
        n_instances: *mut c_int,
        instances: *mut *mut PK_INSTANCE_t,
    ) -> PK_ERROR_code_t;

    /// Return matched arrays of parts and their transforms for the direct
    /// instances of this assembly.
    pub fn PK_ASSEMBLY_ask_parts_transfs(
        assembly: PK_ASSEMBLY_t,
        n_parts: *mut c_int,
        parts: *mut *mut PK_PART_t,
        transfs: *mut *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    // ---- Instance creation & modification -----------------------------------

    /// Create an instance within an assembly, referencing a part with a
    /// positioning transform. The transform must be a rigid motion or
    /// reflection and must not be shared with another instance.
    pub fn PK_INSTANCE_create(
        instance_sf: *mut PK_INSTANCE_sf_t,
        instance: *mut PK_INSTANCE_t,
    ) -> PK_ERROR_code_t;

    /// Change the part referenced by an instance.
    pub fn PK_INSTANCE_change_part(
        instance: PK_INSTANCE_t,
        part: PK_PART_t,
    ) -> PK_ERROR_code_t;

    /// Replace the transform of an instance. The old transform entity is
    /// deleted and the new one is adopted.
    pub fn PK_INSTANCE_replace_transf(
        instance: PK_INSTANCE_t,
        transf: PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Transform an instance by composing a transformation onto its existing
    /// transform.
    pub fn PK_INSTANCE_transform(
        instance: PK_INSTANCE_t,
        transf: PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    // ---- Instance query -----------------------------------------------------

    /// Return the standard form of an instance: owning assembly, referenced
    /// part, and positioning transform.
    pub fn PK_INSTANCE_ask(
        instance: PK_INSTANCE_t,
        instance_sf: *mut PK_INSTANCE_sf_t,
    ) -> PK_ERROR_code_t;

    // ---- Part reverse-lookup ------------------------------------------------

    /// Return all instances that reference a given part.
    pub fn PK_PART_ask_ref_instances(
        part: PK_PART_t,
        n_instances: *mut c_int,
        instances: *mut *mut PK_INSTANCE_t,
    ) -> PK_ERROR_code_t;
}
