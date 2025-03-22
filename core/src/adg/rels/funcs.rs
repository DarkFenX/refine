use crate::{adg::rels::KeyPart, ed};

// Since CCP data is full of dead references to various entities with value 0, I assume it stands
// for "no reference"
pub(super) fn attr_val_to_fk(val: ed::EAttrVal) -> Option<KeyPart> {
    match val == 0.0 {
        true => None,
        false => Some(val.round() as KeyPart),
    }
}
