use crate::{adg::rels::KeyPart, defs::AttrVal};

// Since CCP data is full of dead references to various entities with value 0, I assume it stands
// for "no reference"
pub(super) fn attrval_to_fk(val: Option<AttrVal>) -> Option<KeyPart> {
    val.and_then(|v| if v == 0.0 { None } else { Some(v.round() as KeyPart) })
}
