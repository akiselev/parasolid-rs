//! Distance, range, clash detection, and intersection functions.
//!
//! Bindings for Parasolid distance/range (Ch. 26), clash detection (Ch. 27),
//! and intersection functions (Ch. 54).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_double, c_int};

use crate::*;

// =============================================================================
// Range enums
// =============================================================================

/// Whether to find minimum or maximum distance.
pub type PK_range_type_t = c_int;
/// Find minimum distance (default).
pub const PK_range_type_minimum_c: PK_range_type_t = 22820;
/// Find maximum distance.
pub const PK_range_type_maximum_c: PK_range_type_t = 22821;

/// Optimization level for range computation.
pub type PK_range_opt_t = c_int;
/// Optimize for performance (default); may return local extremum.
pub const PK_range_opt_performance_c: PK_range_opt_t = 23760;
/// Optimize for accuracy; slower but more reliable global result.
pub const PK_range_opt_accuracy_c: PK_range_opt_t = 23761;

/// Result status of a range computation.
pub type PK_range_result_t = c_int;
/// Min/max distance successfully found.
pub const PK_range_result_found_c: PK_range_result_t = 18270;
/// No distance greater than supplied lower_bound found.
pub const PK_range_result_lower_c: PK_range_result_t = 18271;
/// No distance less than supplied upper_bound found.
pub const PK_range_result_upper_c: PK_range_result_t = 18272;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_range_result_not_found_c: PK_range_result_t = 18273;

/// Type of initial estimate supplied to a range function.
pub type PK_range_guess_t = c_int;
/// No estimate (default).
pub const PK_range_guess_no_c: PK_range_guess_t = 18260;
/// Parameter estimate.
pub const PK_range_guess_param_c: PK_range_guess_t = 18261;
/// Position estimate.
pub const PK_range_guess_vector_c: PK_range_guess_t = 18262;

/// Which entity level the endpoint on the found sub-topology refers to
/// (`PK_TOPOL_range_vector`).
pub type PK_range_param_entity_t = c_int;
/// Report the containing topological entity (default).
pub const PK_range_param_entity_topol_c: PK_range_param_entity_t = 24990;
/// Report the sub-entity.
pub const PK_range_param_entity_sub_c: PK_range_param_entity_t = 24991;

// =============================================================================
// Range helper structures
// =============================================================================

/// Initial estimate for a range computation.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_guess_s_t {
    /// Type of estimate.
    pub guess_type: PK_range_guess_t,
    /// Parameter values (up to 2: one for curve/edge, two for surface/face).
    pub parameters: [c_double; 2],
    /// Position vector (used when `guess_type == PK_range_guess_vector_c`).
    pub vector: PK_VECTOR_t,
}

/// Parametric bounds for a geometrical entity used in range functions.
///
/// **[journal-recovered]** from `PKU_journal_range_param_bound` (V37.01.243) —
/// 40 bytes, and *not* the `{interval, uvbox}` pair the previous definition
/// used (which was 48 bytes and put both members side by side). The real shape
/// is a flag, a class discriminator, and a **union**: class `0x204` selects the
/// interval form, anything else the uvbox form.
///
/// ```text
/// @0  have_param_bound   (PK_LOGICAL_t)
/// @4  param_bound_class  (0x204 = interval, else uvbox)
/// @8  union { PK_INTERVAL_t (16B) | PK_UVBOX_t (32B) }
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_param_bound_t {
    pub have_param_bound: PK_LOGICAL_t, // @0
    pub param_bound_class: c_int,       // @4
    /// Union storage; interpret per `param_bound_class`.
    pub bound: [c_double; 4], // @8 (32 bytes, the larger member)
}

/// `param_bound_class` value selecting the interval (curve) form. [probed]
pub const PK_range_param_bound_class_interval_c: c_int = 0x204;

/// Details of one endpoint in a range result — **56 bytes**.
///
/// **[journal-recovered]** from `PKU_journal_range_end` (V37.01.243). The
/// previous binding stopped after `parameters` and declared 48 bytes, missing
/// the two trailing logicals. That under-sized every enclosing result struct
/// and put `PK_range_2_r_t::end_2` at the wrong offset.
///
/// ```text
/// @0  entity        @4  sub_entity
/// @8  position      (PK_VECTOR_t, 24B)
/// @32 parameters[0] @40 parameters[1]
/// @48 region        (PK_LOGICAL_t, 1 byte)
/// @49 negative      (PK_LOGICAL_t, 1 byte)
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_end_t {
    /// Entity tag at this endpoint.
    pub entity: PK_ENTITY_t, // @0
    /// Sub-entity tag (face/edge/vertex on which the closest point lies).
    pub sub_entity: PK_ENTITY_t, // @4
    /// Position of the endpoint.
    pub position: PK_VECTOR_t, // @8
    /// Parameter values at the endpoint (1 for curve/edge, 2 for surface/face).
    pub parameters: [c_double; 2], // @32
    /// Whether the endpoint lies in a region rather than on a boundary.
    /// **Single byte** — the journal reads it at 0x30.
    pub region: u8, // @48
    /// Whether the reported distance is negative (inside).
    /// **Single byte** — the journal reads it at 0x31.
    pub negative: u8, // @49
    _pad: [u8; 6], // @50 — pad to the 8-byte alignment of the doubles above
}

const _: () = {
    assert!(core::mem::size_of::<PK_range_end_t>() == 56);
};

/// Result of a range computation between two entities — **120 bytes**.
///
/// NOTE (decompile-verified): the r_t does **not** carry a status field — the
/// status is the separate `range_result` out-param. `distance` is at offset 0.
///
/// `end_2` sits at **@64**, not @56: `PKU_journal_range_2_r` walks an
/// `undefined8*` and reads `ends[0]` at `param_1 + 1` (byte 8) and `ends[1]` at
/// `param_1 + 8` (byte 64). With the old 48-byte `PK_range_end_t` the whole
/// struct was 104 bytes and the kernel overran the caller's stack by 16.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_2_r_t {
    /// Computed distance.
    pub distance: c_double, // @0
    /// Details for the first entity endpoint.
    pub end_1: PK_range_end_t, // @8
    /// Details for the second entity endpoint. 8 + 56 = 64, no padding.
    pub end_2: PK_range_end_t, // @64
}

const _: () = {
    assert!(core::mem::size_of::<PK_range_2_r_t>() == 120);
};

/// Result of a range computation between an entity and a position vector —
/// **64 bytes** (the kernel writes 64; the old declaration claimed 56).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_1_r_t {
    /// Computed distance.
    pub distance: c_double, // @0
    /// Details for the entity endpoint.
    pub end: PK_range_end_t, // @8
}

const _: () = {
    assert!(core::mem::size_of::<PK_range_1_r_t>() == 64);
};

// =============================================================================
// Range options structures
// =============================================================================

/// Optional lower/upper distance bounds for a range computation.
///
/// **A 32-byte STRUCT, not the `PK_bound_t` enum** — recovered by decompiling
/// the field validator `FUN_181106a20`: it reads `have_lower_bound`/
/// `have_upper_bound` as 0/1 flags and the two doubles only when the flag is
/// TRUE. All-zero == "no bound" and validates cleanly (the default).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_bound_t {
    pub have_upper_bound: PK_LOGICAL_t, // @0
    pub upper_bound: c_double,          // @8
    pub have_lower_bound: PK_LOGICAL_t, // @16
    pub lower_bound: c_double,          // @24
} // 32 bytes

// =============================================================================
// o_t_version and DEAD FIELDS
//
// The kernel migrates a caller's options struct from the stamped `o_t_version`
// into its current internal form, copying ONLY the fields that version defines
// and overwriting the rest with hard-coded defaults. Stamping version 1 on
// these structs therefore makes several fields **silently unread**:
//
//   PK_TOPOL_range_o_t / PK_GEOM_range_o_t
//     v1: range_type and opt_level forced (minimum, performance)
//     v2: adds range_type;  v3: adds opt_level + param_bound
//   PK_TOPOL_range_vector_o_t
//     v1: opt_level and param_entity forced
//     v2: adds opt_level;   v3: adds param_entity
//   PK_GEOM_range_vector_o_t   (max version 2)
//     v1: opt_level forced; v2: adds opt_level
//
// Measured consequence of the old `o_t_version: 1`: asking for
// PK_range_type_maximum_c silently returned the MINIMUM distance with no error,
// an illegal opt_level token was accepted without complaint, and a supplied
// param_bound was ignored while the call still reported `found`.
//
// The defaults below stamp the HIGHEST version each entry point accepts, so
// every field these structs expose is actually read. Verified by sweeping
// accepted versions at runtime; see docs/option-version-protocol.md.
// =============================================================================

impl Default for PK_range_bound_t {
    fn default() -> Self {
        Self {
            have_lower_bound: PK_LOGICAL_false,
            lower_bound: 0.0,
            have_upper_bound: PK_LOGICAL_false,
            upper_bound: 0.0,
        }
    }
}

/// Options for entity-to-entity range functions (`PK_TOPOL_range`).
///
/// Authoritative **152-byte** layout recovered by decompiling `PK_TOPOL_range`
/// (V37.01.243): `bound` is a 32-byte `PK_range_bound_t` @16, the two `guesses`
/// are 48-byte `PK_range_guess_s_t` @48/@96, `range_type`@144, `opt_level`@148.
/// The catalog's 40-byte `bound:int` layout was wrong (that dead-ended on err 908).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_range_o_t {
    pub o_t_version: c_int,               // @0
    pub have_tolerance: PK_LOGICAL_t,     // @4
    pub tolerance: c_double,              // @8
    pub bound: PK_range_bound_t,          // @16  (32B)
    pub guesses: [PK_range_guess_s_t; 2], // @48  (96B)
    pub range_type: PK_range_type_t,      // @144
    pub opt_level: PK_range_opt_t,        // @148
} // 152 bytes

impl Default for PK_TOPOL_range_o_t {
    /// `o_t_version = 3` — confirmed still the ceiling by
    /// `version_upgrade_probe` on V37.01.243: v1..=3 return rc 0, v4+ return
    /// `o_t_version_unknown` (5022), and a garbage `opt_level` (the last field
    /// of this struct) returns `field_of_wrong_type` (5014) at v3 only. So v3
    /// reads through to the end of this layout and nothing higher exists.
    fn default() -> Self {
        Self {
            o_t_version: 3,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
            bound: PK_range_bound_t::default(),
            guesses: [PK_range_guess_s_t {
                guess_type: PK_range_guess_no_c,
                parameters: [0.0, 0.0],
                vector: [0.0, 0.0, 0.0],
            }; 2],
            range_type: PK_range_type_minimum_c,
            opt_level: PK_range_opt_accuracy_c,
        }
    }
}

/// Options for entity-to-vector range functions (`PK_TOPOL_range_vector`).
///
/// Authoritative **104-byte** layout recovered by decompiling
/// `PK_TOPOL_range_vector`: `bound` is a 32-byte struct @16, `guess` a 48-byte
/// `PK_range_guess_s_t` @48, `opt_level`@96, `param_entity`@100.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_range_vector_o_t {
    pub o_t_version: c_int,                    // @0
    pub have_tolerance: PK_LOGICAL_t,          // @4
    pub tolerance: c_double,                   // @8
    pub bound: PK_range_bound_t,               // @16 (32B)
    pub guess: PK_range_guess_s_t,             // @48 (48B)
    pub opt_level: PK_range_opt_t,             // @96
    pub param_entity: PK_range_param_entity_t, // @100
} // 104 bytes

impl Default for PK_TOPOL_range_vector_o_t {
    /// `o_t_version = 3` — confirmed the ceiling: v1..=3 rc 0, v4+ 5022, and a
    /// garbage `param_entity` (last field) gives 5014 at v3 only.
    fn default() -> Self {
        Self {
            o_t_version: 3,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
            bound: PK_range_bound_t::default(),
            guess: PK_range_guess_s_t {
                guess_type: PK_range_guess_no_c,
                parameters: [0.0, 0.0],
                vector: [0.0, 0.0, 0.0],
            },
            opt_level: PK_range_opt_accuracy_c,
            param_entity: PK_range_param_entity_topol_c,
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<PK_range_bound_t>() == 32);
    assert!(core::mem::size_of::<PK_range_guess_s_t>() == 48);
    assert!(core::mem::size_of::<PK_TOPOL_range_o_t>() == 152);
    assert!(core::mem::size_of::<PK_TOPOL_range_vector_o_t>() == 104);
};

/// Options for `PK_GEOM_range` — **[journal-recovered]**, 240 bytes.
///
/// The previous definition had the fields in the wrong order and the wrong
/// sizes. Real layout, from `PK_GEOM_range`'s own journalling:
///
/// ```text
/// @0   o_t_version        @4   have_tolerance     @8   tolerance
/// @16  bound (32B)        @48  guesses[0] (48B)   @96  guesses[1] (48B)
/// @144 range_type         @152 param_bound[0] (40B)
/// @192 param_bound[1] (40B)                       @232 opt_level
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_range_o_t {
    pub o_t_version: c_int,                       // @0
    pub have_tolerance: PK_LOGICAL_t,             // @4
    pub tolerance: c_double,                      // @8
    pub bound: PK_range_bound_t,                  // @16
    pub guesses: [PK_range_guess_s_t; 2],         // @48
    pub range_type: PK_range_type_t,              // @144
    _pad: c_int,                                  // @148
    pub param_bound: [PK_range_param_bound_t; 2], // @152
    pub opt_level: PK_range_opt_t,                // @232
}

const _: () = {
    assert!(core::mem::size_of::<PK_GEOM_range_o_t>() == 240);
};

impl Default for PK_GEOM_range_o_t {
    /// `o_t_version = 3` — confirmed the ceiling: v1..=3 rc 0, v4+ 5022, and a
    /// garbage `opt_level` (last field) gives 5014 at v3 only.
    fn default() -> Self {
        let no_guess = PK_range_guess_s_t {
            guess_type: PK_range_guess_no_c,
            parameters: [0.0, 0.0],
            vector: [0.0, 0.0, 0.0],
        };
        let no_pbound = PK_range_param_bound_t {
            have_param_bound: PK_LOGICAL_false,
            param_bound_class: PK_range_param_bound_class_interval_c,
            bound: [0.0; 4],
        };
        Self {
            o_t_version: 3,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
            bound: PK_range_bound_t::default(),
            guesses: [no_guess, no_guess],
            range_type: PK_range_type_minimum_c,
            _pad: 0,
            param_bound: [no_pbound, no_pbound],
            opt_level: PK_range_opt_accuracy_c,
        }
    }
}

/// Options for `PK_GEOM_range_vector` — **[journal-recovered]**, 104 bytes.
///
/// The previous definition listed `opt_level` before `guess` and carried a
/// `param_bound` the kernel does not read here; passing it produced
/// `PK_ERROR_field_of_wrong_type` (5014) naming `local_opts`. Real layout
/// matches `PK_TOPOL_range_vector_o_t` minus its trailing `param_entity`:
///
/// ```text
/// @0  o_t_version  @4 have_tolerance  @8 tolerance
/// @16 bound (32B)  @48 guess (48B)    @96 opt_level
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_range_vector_o_t {
    pub o_t_version: c_int,           // @0
    pub have_tolerance: PK_LOGICAL_t, // @4
    pub tolerance: c_double,          // @8
    pub bound: PK_range_bound_t,      // @16
    pub guess: PK_range_guess_s_t,    // @48
    pub opt_level: PK_range_opt_t,    // @96
}

const _: () = {
    assert!(core::mem::size_of::<PK_GEOM_range_vector_o_t>() == 104);
};

impl Default for PK_GEOM_range_vector_o_t {
    /// `o_t_version = 2` — confirmed the ceiling: v1/v2 rc 0, v3+ 5022, and a
    /// garbage `opt_level` (last field) gives 5014 at v2 only. Note this
    /// family member tops out one lower than the three `range`/`range_vector`
    /// siblings above, which reach 3.
    fn default() -> Self {
        // Zero is NOT a valid token for `guess_type` or `opt_level`; leaving
        // them zeroed makes the kernel reject the whole struct with
        // PK_ERROR_field_of_wrong_type (5014) naming `local_opts`.
        Self {
            o_t_version: 2,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
            bound: PK_range_bound_t::default(),
            guess: PK_range_guess_s_t {
                guess_type: PK_range_guess_no_c,
                parameters: [0.0, 0.0],
                vector: [0.0, 0.0, 0.0],
            },
            opt_level: PK_range_opt_accuracy_c,
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<PK_range_bound_t>() == 32);
    assert!(core::mem::size_of::<PK_range_param_bound_t>() == 40);
    assert!(core::mem::size_of::<PK_range_guess_s_t>() == 48);
};

/// Options for local range functions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_range_local_o_t {
    pub o_t_version: c_int,
    /// Whether a tolerance is supplied.
    pub have_tolerance: PK_LOGICAL_t,
    /// Accuracy tolerance.
    pub tolerance: c_double,
    /// Initial estimate for entity 1.
    pub guess_1: PK_range_guess_s_t,
    /// Initial estimate for entity 2.
    pub guess_2: PK_range_guess_s_t,
    /// Parametric bounds for entity 1.
    pub param_bound_1: PK_range_param_bound_t,
    /// Parametric bounds for entity 2.
    pub param_bound_2: PK_range_param_bound_t,
}

// =============================================================================
// Clash detection enums and structures
// =============================================================================

/// Classification of a clash between two topological entities.
///
/// **[probed]** — recovered by running `PK_TOPOL_clash` over known
/// configurations (`crates/parasolid-test/src/bin/range_probe.rs`). The
/// previous constants were plain 0..4 and **wrong**, the same fabricated-enum
/// shape as the old `PK_ERROR_*` table.
///
/// | configuration | observed token |
/// |---|---:|
/// | identical blocks (full overlap) | 7 |
/// | partially overlapping blocks | 7 |
/// | blocks sharing exactly one face | 4 |
/// | small block strictly inside a large one | 2 |
/// | disjoint blocks | *no records* |
///
/// Only these three values have been observed. The containment case was not
/// probed in both directions, so no separate `b_in_a` constant is claimed.
pub type PK_TOPOL_clash_type_t = c_int;

/// Entities share common interior (overlap, partial or total). [probed]
pub const PK_TOPOL_clash_interfere_c: PK_TOPOL_clash_type_t = 7;
/// Entities touch but share no common interior. [probed]
pub const PK_TOPOL_clash_abut_c: PK_TOPOL_clash_type_t = 4;
/// One entity lies strictly inside the other. [probed]
pub const PK_TOPOL_clash_contained_c: PK_TOPOL_clash_type_t = 2;

/// Options for `PK_TOPOL_clash`.
///
/// **[journal-recovered]** from `PKU_journal_TOPOL_clash_o` (V37.01.243). The
/// previous definition was wrong in two ways that matter: it omitted the three
/// leading exception-list fields entirely, and it modelled the four logicals as
/// 4-byte ints when the kernel packs them as **single bytes** at 24..27. Every
/// field after `o_t_version` was therefore at the wrong offset.
///
/// ```text
/// @0  o_t_version
/// @4  n_op_ex               number of excepted topology pairs
/// @8  op_ex1                PK_TOPOL_t* — exception list, side 1
/// @16 op_ex2                PK_TOPOL_t* — exception list, side 2
/// @24 find_all              (1 byte)
/// @25 find_intersect        (1 byte)
/// @26 mul_target_tf         (1 byte)
/// @27 mul_tool_tf           (1 byte)
/// @28 target_owner
/// @32 tool_owner
/// @36 n_parts_with_scales
/// @40 parts_with_scales     PK_PART_t*
/// @48 scale_factors
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_clash_o_t {
    pub o_t_version: c_int,        // @0
    pub n_op_ex: c_int,            // @4
    pub op_ex1: *const PK_TOPOL_t, // @8
    pub op_ex2: *const PK_TOPOL_t, // @16
    /// Find all clashes, or stop after the first (default).
    pub find_all: u8, // @24
    /// Classify each clash; populates `clash_type` in the result elements.
    pub find_intersect: u8, // @25
    /// Supply per-target transforms.
    pub mul_target_tf: u8, // @26
    /// Supply per-tool transforms.
    pub mul_tool_tf: u8, // @27
    /// Owning body of the targets (for face-level classification).
    pub target_owner: PK_BODY_t, // @28
    /// Owning body of the tools.
    pub tool_owner: PK_BODY_t, // @32
    pub n_parts_with_scales: c_int, // @36
    pub parts_with_scales: *const PK_PART_t, // @40
    pub scale_factors: *const c_double, // @48
}

const _: () = {
    assert!(core::mem::size_of::<PK_TOPOL_clash_o_t>() == 56);
};

impl Default for PK_TOPOL_clash_o_t {
    /// `o_t_version = 3` — confirmed the ceiling: v1..=3 return rc 0 and v4+
    /// return `o_t_version_unknown` (5022).
    ///
    /// Unlike the range family there is no positive "last field is read" proof
    /// here: the trailing fields are a count and two pointers, and an
    /// out-of-range `n_parts_with_scales` is not type-checked (it is an int,
    /// not an enum token), so no garbage value produces 5014. The ceiling
    /// itself is solid; the completeness of the tail is inferred from the
    /// struct being accepted at the top of the band.
    fn default() -> Self {
        Self {
            o_t_version: 3,
            n_op_ex: 0,
            op_ex1: core::ptr::null(),
            op_ex2: core::ptr::null(),
            find_all: 0,
            find_intersect: 0,
            mul_target_tf: 0,
            mul_tool_tf: 0,
            target_owner: PK_ENTITY_null,
            tool_owner: PK_ENTITY_null,
            n_parts_with_scales: 0,
            parts_with_scales: core::ptr::null(),
            scale_factors: core::ptr::null(),
        }
    }
}

/// One clash reported by `PK_TOPOL_clash` — **[decompile-recovered]**, 20 bytes.
///
/// `PK_TOPOL_clash_t` was previously a bare `c_int` typedef, so the returned
/// array could not be read at all. The journalling loop in the real
/// `PK_TOPOL_clash` (export @18043f9a0) walks the array with a stride of
/// **5 ints**, emitting `target`, `target_index`, `tool`, `tool_index`,
/// `clash_type` in that order.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_clash_rec_t {
    /// The clashing target topology.
    pub target: PK_TOPOL_t, // @0
    /// Index of that target in the caller's `targets` array.
    pub target_index: c_int, // @4
    /// The clashing tool topology.
    pub tool: PK_TOPOL_t, // @8
    /// Index of that tool in the caller's `tools` array.
    pub tool_index: c_int, // @12
    /// Classification, populated when `find_intersect` is set.
    pub clash_type: PK_TOPOL_clash_type_t, // @16
}

const _: () = {
    assert!(core::mem::size_of::<PK_TOPOL_clash_rec_t>() == 20);
};

/// Result structure for `PK_TOPOL_clash`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_clash_r_t {
    /// Number of clashes found.
    pub n_clashes: c_int,
    /// Array of target entities involved in clashes.
    pub targets: *mut PK_TOPOL_t,
    /// Array of tool entities involved in clashes.
    pub tools: *mut PK_TOPOL_t,
    /// Array of clash type classifications (populated when `find_intersect` is set).
    pub clash_types: *mut PK_TOPOL_clash_type_t,
}

// =============================================================================
// Intersection options structures
// =============================================================================

/// Options for `PK_CURVE_intersect_curve`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_intersect_curve_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
    /// Surface containing both curves (for parametric-space intersection).
    pub common_surf: PK_SURF_t,
}

/// Options for `PK_SURF_intersect_curve`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_intersect_curve_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
    /// Reserved (`interest`) — kept for correct v1 struct size (64 bytes).
    pub _interest_reserved: c_int,
}

/// Options for `PK_FACE_intersect_face`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_intersect_face_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
    /// Parameter box for face 1.
    pub have_uvbox_1: PK_LOGICAL_t,
    pub uvbox_1: PK_UVBOX_t,
    /// Parameter box for face 2.
    pub have_uvbox_2: PK_LOGICAL_t,
    pub uvbox_2: PK_UVBOX_t,
    /// Point of interest to seed the intersection.
    pub have_vector: PK_LOGICAL_t,
    pub vector: PK_VECTOR_t,
    /// Mixed-dimension curve category (`PK_mixed_intersection_t`).
    pub mixed_curve_category: c_int,
    /// Intersection tolerance.
    pub tolerance: c_double,
    /// Reserved (`use`) — kept for correct v1 struct size (192 bytes).
    pub _use_reserved: c_int,
}

/// Which curve representation the intersector may produce for mixed-dimension
/// results (`PK_SURF_intersect_surf_o_t::mixed_curve_category` and friends).
///
/// **Zero is not a legal value.** The field is version-gated, so a zeroed
/// options struct is accepted at low `o_t_version` and rejected once the kernel
/// starts reading the field — the same trap that made `range_type` a dead field
/// in Stage 6. Values from `parasolid-re/catalog/pk-enums.h`.
pub type PK_mixed_intersection_t = c_int;
/// Produce pline (polyline) curves for mixed-dimension intersections.
pub const PK_mixed_intersection_pline_c: PK_mixed_intersection_t = 26650;
/// Produce classic (analytic/spline) curves.
pub const PK_mixed_intersection_classic_c: PK_mixed_intersection_t = 26651;
/// Produce both representations.
pub const PK_mixed_intersection_both_c: PK_mixed_intersection_t = 26652;

/// Options for `PK_SURF_intersect_surf`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_intersect_surf_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
    /// Parameter box for surface 1.
    pub have_uvbox_1: PK_LOGICAL_t,
    pub uvbox_1: PK_UVBOX_t,
    /// Parameter box for surface 2.
    pub have_uvbox_2: PK_LOGICAL_t,
    pub uvbox_2: PK_UVBOX_t,
    /// Point of interest to seed the intersection.
    pub have_vector: PK_LOGICAL_t,
    pub vector: PK_VECTOR_t,
    /// Mixed-dimension curve category (`PK_mixed_intersection_t`).
    pub mixed_curve_category: c_int,
    /// Intersection tolerance.
    pub tolerance: c_double,
    /// Reserved (`use`) — kept for correct v1 struct size (192 bytes).
    pub _use_reserved: c_int,
}

// Intersection classification tokens.
//
// `PK_intersect_curve_t` is now a CLOSED, fully enumerated set: the RE catalog
// (`pk-enums.h`) defines exactly two members, `simple` 14651 and `tangent`
// 14652, and a 15-configuration runtime scan — analytic pairs, tangent
// plane/cylinder, torus crown, cone-sphere tangency, saddle B-surface planes,
// offset B-surfaces, spun and swept surfaces, the Villarceau bitangent plane,
// lemon/apple/horn tori, cone apex and seam planes — produced nothing else.
// The earlier "treat other values as opaque" hedge can be retired for this
// type. (The sibling `PK_intersect_fc_t` has 13 members and is NOT closed.)

/// Type of an intersection curve from the surf/face intersection functions
/// (`PK_intersect_curve_t`).
pub type PK_intersect_curve_t = c_int;
/// A transversal (clean, non-tangential) intersection curve. [dynamic-observed]
/// Seen for plane∩plane (line), cyl∩plane (circle), face∩face, face∩surf.
pub const PK_intersect_curve_simple_c: PK_intersect_curve_t = 14651; // 0x393b
/// A tangential intersection curve (the surfaces touch without crossing).
/// [dynamic-observed] Seen for a plane tangent to a cylinder (tangent line).
pub const PK_intersect_curve_tangent_c: PK_intersect_curve_t = 14652; // 0x393c

/// Type of a point intersection from `PK_CURVE_intersect_curve` /
/// `PK_SURF_intersect_curve` (`PK_intersect_vector_t`).
pub type PK_intersect_vector_t = c_int;
/// A transversal point intersection. [dynamic-observed] Seen for curve∩curve
/// (two lines crossing) and surf∩curve (line piercing a plane).
pub const PK_intersect_vector_simple_c: PK_intersect_vector_t = 14611; // 0x3913
// [re-abi] appended 3 missing member(s) from pk-enums.h
pub const PK_intersect_vector_tangent_c: PK_intersect_vector_t = 14612;
pub const PK_intersect_vector_start_c: PK_intersect_vector_t = 14613;
pub const PK_intersect_vector_end_c: PK_intersect_vector_t = 14614;

/// Type of a face/curve point intersection from `PK_FACE_intersect_curve`
/// (`PK_intersect_fc_t`).
pub type PK_intersect_fc_t = c_int;
/// A transversal face/curve point intersection. [dynamic-observed] Seen for a
/// line piercing a planar face.
pub const PK_intersect_fc_simple_c: PK_intersect_fc_t = 14801; // 0x39d1
// [re-abi] appended 12 missing member(s) from pk-enums.h
pub const PK_intersect_fc_tangent_c: PK_intersect_fc_t = 14802;
pub const PK_intersect_fc_out_in_c: PK_intersect_fc_t = 14803;
pub const PK_intersect_fc_in_out_c: PK_intersect_fc_t = 14804;
pub const PK_intersect_fc_out_coi_c: PK_intersect_fc_t = 14805;
pub const PK_intersect_fc_coi_out_c: PK_intersect_fc_t = 14806;
pub const PK_intersect_fc_coi_in_c: PK_intersect_fc_t = 14807;
pub const PK_intersect_fc_in_coi_c: PK_intersect_fc_t = 14808;
pub const PK_intersect_fc_in_tangent_c: PK_intersect_fc_t = 14809;
pub const PK_intersect_fc_out_tangent_c: PK_intersect_fc_t = 14810;
pub const PK_intersect_fc_in_c: PK_intersect_fc_t = 14811;
pub const PK_intersect_fc_start_c: PK_intersect_fc_t = 14812;
pub const PK_intersect_fc_end_c: PK_intersect_fc_t = 14813;

/// Options for `PK_FACE_intersect_surf`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_intersect_surf_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
    /// Parameter box for the face.
    pub have_uvbox_1: PK_LOGICAL_t,
    pub uvbox_1: PK_UVBOX_t,
    /// Parameter box for the surface.
    pub have_uvbox_2: PK_LOGICAL_t,
    pub uvbox_2: PK_UVBOX_t,
    /// Point of interest to seed the intersection.
    pub have_vector: PK_LOGICAL_t,
    pub vector: PK_VECTOR_t,
    /// Mixed-dimension curve category (`PK_mixed_intersection_t`).
    pub mixed_curve_category: c_int,
    /// Intersection tolerance.
    pub tolerance: c_double,
    /// Reserved (`use`) — kept for correct v1 struct size (192 bytes).
    pub _use_reserved: c_int,
}

// =============================================================================
// Extern function declarations — Range (Chapter 26)
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // ---- Standard range (entity-to-entity) ----

    /// Global min/max distance between two geometrical entities.
    pub fn PK_GEOM_range(
        geom_1: PK_GEOM_t,
        geom_2: PK_GEOM_t,
        options: *mut PK_GEOM_range_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between two topological entities.
    pub fn PK_TOPOL_range(
        topol_1: PK_TOPOL_t,
        topol_2: PK_TOPOL_t,
        options: *mut PK_TOPOL_range_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between a topological entity and a geometric entity.
    pub fn PK_TOPOL_range_geom(
        topol: PK_TOPOL_t,
        geom: PK_GEOM_t,
        options: *mut PK_TOPOL_range_geom_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Array range (array-to-array) ----

    /// Global min/max distance between two arrays of geometrical entities.
    pub fn PK_GEOM_range_array(
        n_geoms_1: c_int,
        geoms_1: *mut PK_GEOM_t,
        n_geoms_2: c_int,
        geoms_2: *mut PK_GEOM_t,
        options: *mut PK_GEOM_range_array_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between two arrays of topological entities.
    pub fn PK_TOPOL_range_array(
        n_topols_1: c_int,
        topols_1: *mut PK_TOPOL_t,
        n_topols_2: c_int,
        topols_2: *mut PK_TOPOL_t,
        options: *mut PK_TOPOL_range_array_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between arrays of topological and geometric entities.
    pub fn PK_TOPOL_range_geom_array(
        n_topols: c_int,
        topols: *mut PK_TOPOL_t,
        n_geoms: c_int,
        geoms: *mut PK_GEOM_t,
        options: *mut PK_TOPOL_range_geom_array_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Vector range (entity-to-position) ----

    /// Global min distance between a geometrical entity and a position.
    pub fn PK_GEOM_range_vector(
        geom: PK_GEOM_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_GEOM_range_vector_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distances between a geometrical entity and an array of positions.
    pub fn PK_GEOM_range_vector_many(
        geom: PK_GEOM_t,
        n_vectors: c_int,
        vectors: *mut PK_VECTOR_t,
        options: *mut PK_GEOM_range_vector_many_o_t,
        range_results: *mut PK_range_result_t,
        ranges: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distance between a topological entity and a position.
    pub fn PK_TOPOL_range_vector(
        topol: PK_TOPOL_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_TOPOL_range_vector_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distance between an array of geometrical entities and a position.
    pub fn PK_GEOM_range_array_vector(
        n_geoms: c_int,
        geoms: *mut PK_GEOM_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_GEOM_range_array_vector_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distance between an array of topological entities and a position.
    pub fn PK_TOPOL_range_array_vector(
        n_topols: c_int,
        topols: *mut PK_TOPOL_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_TOPOL_range_array_vector_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Local range ----

    /// Local min distance between two geometrical entities.
    pub fn PK_GEOM_range_local(
        geom_1: PK_GEOM_t,
        geom_2: PK_GEOM_t,
        options: *mut PK_GEOM_range_local_o_t,
        n_ranges: *mut c_int,
        ranges: *mut *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Local min distance between two topological entities.
    pub fn PK_TOPOL_range_local(
        topol_1: PK_TOPOL_t,
        topol_2: PK_TOPOL_t,
        options: *mut PK_TOPOL_range_local_o_t,
        n_ranges: *mut c_int,
        ranges: *mut *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Local min distance between a geometrical entity and a position.
    pub fn PK_GEOM_range_local_vector(
        geom: PK_GEOM_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_GEOM_range_local_vector_o_t,
        n_ranges: *mut c_int,
        ranges: *mut *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Local min/max distance between a topological entity and a position.
    pub fn PK_TOPOL_range_local_vector(
        topol: PK_TOPOL_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_TOPOL_range_local_vector_o_t,
        n_ranges: *mut c_int,
        ranges: *mut *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Clash detection (Chapter 27)
    // =========================================================================

    /// Detect clashes between two sets of topological entities.
    ///
    /// Receives target and tool topology sets, returns clashing entity pairs
    /// and optional classification.
    pub fn PK_TOPOL_clash(
        n_targets: c_int,
        targets: *mut PK_TOPOL_t,
        tf1: *mut PK_TRANSF_t,
        n_tools: c_int,
        tools: *mut PK_TOPOL_t,
        tf2: *mut PK_TRANSF_t,
        options: *mut PK_TOPOL_clash_o_t,
        n_clash: *mut c_int,
        clashes: *mut *mut PK_TOPOL_clash_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Intersection functions (Chapter 54)
    // =========================================================================

    /// Find intersections between specified regions of two curves. [documented]
    ///
    /// Outputs: `n_vectors`/`vectors` (positions), `ts_1`/`ts_2` (parameters on
    /// each curve), and `types` (`PK_intersect_vector_t`). The earlier binding
    /// dropped the trailing `types` output, so the kernel wrote it through an
    /// uninitialised pointer.
    pub fn PK_CURVE_intersect_curve(
        curve_1: PK_CURVE_t,
        interval_1: PK_INTERVAL_t,
        curve_2: PK_CURVE_t,
        interval_2: PK_INTERVAL_t,
        options: *const PK_CURVE_intersect_curve_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        ts_1: *mut *mut c_double,
        ts_2: *mut *mut c_double,
        types: *mut *mut PK_intersect_vector_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between a surface and a curve. [documented]
    ///
    /// Outputs: `n_vectors`/`vectors`, `uvs` (surface params), `ts` (curve
    /// params), `types` (`PK_intersect_vector_t`). The earlier binding swapped
    /// the `uvs`/`ts` order and dropped `types`.
    pub fn PK_SURF_intersect_curve(
        surf: PK_SURF_t,
        curve: PK_CURVE_t,
        bounds: PK_INTERVAL_t,
        options: *const PK_SURF_intersect_curve_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        uvs: *mut *mut PK_UV_t,
        ts: *mut *mut c_double,
        types: *mut *mut PK_intersect_vector_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between a face and the specified region of a curve.
    /// No options structure. [documented]
    ///
    /// Outputs: `n_vectors`/`vectors`, `uvs` (face-surface params), `ts` (curve
    /// params), `topols` (topology hit at each point), `types`
    /// (`PK_intersect_fc_t`). The earlier binding swapped `uvs`/`ts` and dropped
    /// the `topols` and `types` outputs.
    pub fn PK_FACE_intersect_curve(
        face: PK_FACE_t,
        curve: PK_CURVE_t,
        bounds: PK_INTERVAL_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        uvs: *mut *mut PK_UV_t,
        ts: *mut *mut c_double,
        topols: *mut *mut PK_TOPOL_t,
        types: *mut *mut PK_intersect_fc_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between two faces. [documented]
    ///
    /// Six outputs, point intersections first then curves, matching
    /// `PK_SURF_intersect_surf`. The earlier binding had only four outputs in
    /// swapped order and dropped `bounds`/`types`.
    pub fn PK_FACE_intersect_face(
        face_1: PK_FACE_t,
        face_2: PK_FACE_t,
        options: *const PK_FACE_intersect_face_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        bounds: *mut *mut PK_INTERVAL_t,
        types: *mut *mut PK_intersect_curve_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between two surfaces.
    ///
    /// Both surfaces must be orphans or from the same body.
    /// Fully coincident surfaces yield no intersection data.
    /// Intersect two surfaces.
    ///
    /// [documented] + [static-observed]: the real signature has **six** output
    /// arguments in this order — point intersections first, then curves with
    /// their parameter bounds and types. The earlier binding had only four
    /// outputs in swapped order (`n_curves, curves, n_points, points`) and was
    /// missing `bounds`/`types`, so the kernel wrote curve bounds/types through
    /// uninitialised pointers. `bounds[i]` is the parameter interval of
    /// `curves[i]`; `types[i]` is its `PK_intersect_curve_t`.
    pub fn PK_SURF_intersect_surf(
        surf_1: PK_SURF_t,
        surf_2: PK_SURF_t,
        options: *const PK_SURF_intersect_surf_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        bounds: *mut *mut PK_INTERVAL_t,
        types: *mut *mut PK_intersect_curve_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between a face and a surface. [documented]
    ///
    /// Six outputs, point intersections first then curves, matching
    /// `PK_SURF_intersect_surf`. The earlier binding had only four outputs in
    /// swapped order and dropped `bounds`/`types`.
    pub fn PK_FACE_intersect_surf(
        face: PK_FACE_t,
        surf: PK_SURF_t,
        options: *const PK_FACE_intersect_surf_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        bounds: *mut *mut PK_INTERVAL_t,
        types: *mut *mut PK_intersect_curve_t,
    ) -> PK_ERROR_code_t;
}
