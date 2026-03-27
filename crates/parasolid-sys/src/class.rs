//! Entity class enquiry and memory management bindings.
//!
//! Covers PK_ENTITY_* class enquiry functions, PK_CLASS_* hierarchy functions,
//! and PK_MEMORY_* allocation/deallocation functions.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::*;
use std::os::raw::{c_int, c_void};

// =============================================================================
// Memory callback function pointer types
// =============================================================================

/// Function pointer type for Parasolid memory allocation callback.
///
/// `size` — number of bytes to allocate.
/// Returns pointer to allocated block, or null on failure.
pub type PK_MEMORY_alloc_f_t = Option<unsafe extern "C" fn(size: c_int) -> *mut c_void>;

/// Function pointer type for Parasolid memory deallocation callback.
///
/// `ptr` — pointer previously returned by the alloc callback.
pub type PK_MEMORY_free_f_t = Option<unsafe extern "C" fn(ptr: *mut c_void)>;

// =============================================================================
// Extern functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {

    // =========================================================================
    // PK_ENTITY — class enquiry
    // =========================================================================

    /// Return the class token for an entity.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_ENTITY_ask_class(PK_ENTITY_t entity, PK_CLASS_t *eclass);
    /// ```
    pub fn PK_ENTITY_ask_class(
        entity: PK_ENTITY_t,
        eclass: *mut PK_CLASS_t,
    ) -> PK_ERROR_code_t;

    /// Test whether an entity is a topological entity.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_ENTITY_is_topol(PK_ENTITY_t entity, PK_LOGICAL_t *answer);
    /// ```
    pub fn PK_ENTITY_is_topol(
        entity: PK_ENTITY_t,
        answer: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Test whether an entity is a geometric entity.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_ENTITY_is_geom(PK_ENTITY_t entity, PK_LOGICAL_t *answer);
    /// ```
    pub fn PK_ENTITY_is_geom(
        entity: PK_ENTITY_t,
        answer: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Test whether an entity is a surface.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_ENTITY_is_surf(PK_ENTITY_t entity, PK_LOGICAL_t *answer);
    /// ```
    pub fn PK_ENTITY_is_surf(
        entity: PK_ENTITY_t,
        answer: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Test whether an entity is a curve.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_ENTITY_is_curve(PK_ENTITY_t entity, PK_LOGICAL_t *answer);
    /// ```
    pub fn PK_ENTITY_is_curve(
        entity: PK_ENTITY_t,
        answer: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Test whether an entity is a part (body or assembly).
    ///
    /// ```c
    /// PK_ERROR_code_t PK_ENTITY_is_part(PK_ENTITY_t entity, PK_LOGICAL_t *answer);
    /// ```
    pub fn PK_ENTITY_is_part(
        entity: PK_ENTITY_t,
        answer: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Delete an entity from the session.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_ENTITY_delete(PK_ENTITY_t entity);
    /// ```
    pub fn PK_ENTITY_delete(entity: PK_ENTITY_t) -> PK_ERROR_code_t;

    /// Delete all attributes attached to an entity.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_ENTITY_delete_attribs(PK_ENTITY_t entity);
    /// ```
    pub fn PK_ENTITY_delete_attribs(entity: PK_ENTITY_t) -> PK_ERROR_code_t;

    /// Copy an entity (legacy version).
    ///
    /// ```c
    /// PK_ERROR_code_t PK_ENTITY_copy(PK_ENTITY_t entity, PK_ENTITY_t *copy);
    /// ```
    pub fn PK_ENTITY_copy(
        entity: PK_ENTITY_t,
        copy: *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // PK_CLASS — hierarchy enquiry
    // =========================================================================

    /// Return the immediate superclass of a class.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_CLASS_ask_superclass(PK_CLASS_t eclass, PK_CLASS_t *superclass);
    /// ```
    pub fn PK_CLASS_ask_superclass(
        eclass: PK_CLASS_t,
        superclass: *mut PK_CLASS_t,
    ) -> PK_ERROR_code_t;

    /// Test whether `eclass` is a subclass of `superclass`.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_CLASS_is_subclass(PK_CLASS_t eclass, PK_CLASS_t superclass, PK_LOGICAL_t *answer);
    /// ```
    pub fn PK_CLASS_is_subclass(
        eclass: PK_CLASS_t,
        superclass: PK_CLASS_t,
        answer: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // PK_MEMORY — allocation and deallocation
    // =========================================================================

    /// Register application-provided memory allocation and deallocation callbacks.
    ///
    /// Parasolid uses these callbacks for all `type **const` return values
    /// (i.e., dynamically-sized arrays allocated by the kernel).
    ///
    /// ```c
    /// PK_ERROR_code_t PK_MEMORY_register_callbacks(
    ///     PK_MEMORY_alloc_f_t alloc_fn,
    ///     PK_MEMORY_free_f_t  free_fn
    /// );
    /// ```
    pub fn PK_MEMORY_register_callbacks(
        alloc_fn: PK_MEMORY_alloc_f_t,
        free_fn: PK_MEMORY_free_f_t,
    ) -> PK_ERROR_code_t;

    /// Query the currently registered memory callbacks.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_MEMORY_ask_callbacks(
    ///     PK_MEMORY_alloc_f_t *alloc_fn,
    ///     PK_MEMORY_free_f_t  *free_fn
    /// );
    /// ```
    pub fn PK_MEMORY_ask_callbacks(
        alloc_fn: *mut PK_MEMORY_alloc_f_t,
        free_fn: *mut PK_MEMORY_free_f_t,
    ) -> PK_ERROR_code_t;

    /// Allocate memory through the registered Parasolid allocator.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_MEMORY_alloc(int size, void **pointer);
    /// ```
    pub fn PK_MEMORY_alloc(
        size: c_int,
        pointer: *mut *mut c_void,
    ) -> PK_ERROR_code_t;

}
