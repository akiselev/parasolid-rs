//! Attributes — application data attached to entities.
//!
//! Parasolid ships system attribute *definitions* (names beginning `SDL/TY`)
//! that applications may use directly. This module wraps definition lookup
//! ([`AttribDef::find`]) and the colour convenience on [`Face`], which attaches
//! an `SDL/TYSA_COLOUR` attribute (field 0 = three RGB doubles in `0.0..=1.0`).

use std::ffi::CString;
use std::os::raw::c_int;

use parasolid_sys::*;

use crate::error::PsResult;
use crate::face::Face;
use crate::memory::PkArray;

/// The system colour attribute definition name (field 0 = RGB doubles).
pub const ATTRIB_COLOUR: &str = "SDL/TYSA_COLOUR";

/// An attribute definition (`PK_ATTDEF_t`) — a template for attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttribDef {
    tag: PK_ATTDEF_t,
}

impl AttribDef {
    pub(crate) fn from_tag(tag: PK_ATTDEF_t) -> Self {
        AttribDef { tag }
    }

    /// Returns the raw PK tag.
    #[inline]
    pub fn tag(&self) -> i32 {
        self.tag
    }

    /// Look up an existing attribute definition by name (e.g. a `SDL/TY…`
    /// system definition). Fails if no definition with that name exists.
    pub fn find(name: &str) -> PsResult<AttribDef> {
        let cname = CString::new(name).expect("attribute name has interior NUL");
        let mut tag: PK_ATTDEF_t = PK_ENTITY_null;
        pk_call!(PK_ATTDEF_find(cname.as_ptr(), &mut tag));
        Ok(AttribDef::from_tag(tag))
    }
}

impl Face {
    /// Attach (or overwrite) this face's colour as RGB in `0.0..=1.0`.
    pub fn set_colour(&self, r: f64, g: f64, b: f64) -> PsResult<()> {
        let attdef = AttribDef::find(ATTRIB_COLOUR)?;
        let mut attrib: PK_ATTRIB_t = PK_ENTITY_null;
        pk_call!(PK_ATTRIB_create_empty(self.tag(), attdef.tag(), &mut attrib));
        let rgb = [r, g, b];
        pk_call!(PK_ATTRIB_set_doubles(attrib, 0, 3, rgb.as_ptr()));
        Ok(())
    }

    /// Read this face's colour if it has an `SDL/TYSA_COLOUR` attribute.
    ///
    /// Returns `None` if the face carries no colour attribute.
    pub fn colour(&self) -> PsResult<Option<(f64, f64, f64)>> {
        let attdef = AttribDef::find(ATTRIB_COLOUR)?;
        // Find colour attributes attached to this face.
        let mut n_attribs: c_int = 0;
        let mut attribs: *mut PK_ATTRIB_t = std::ptr::null_mut();
        pk_call!(PK_ENTITY_ask_attribs(
            self.tag(),
            attdef.tag(),
            &mut n_attribs,
            &mut attribs,
        ));
        let attribs = unsafe { PkArray::from_raw(attribs, n_attribs) };
        let Some(&attrib) = attribs.iter().next() else {
            return Ok(None);
        };
        let mut n_doubles: c_int = 0;
        let mut doubles: *mut f64 = std::ptr::null_mut();
        pk_call!(PK_ATTRIB_ask_doubles(attrib, 0, &mut n_doubles, &mut doubles));
        let vals = unsafe { PkArray::from_raw(doubles, n_doubles) };
        if vals.len() < 3 {
            return Ok(None);
        }
        Ok(Some((vals[0], vals[1], vals[2])))
    }
}
