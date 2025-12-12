use crate::{ad::generator::rels::KeyPart, ed};

// Since CCP data is full of dead references to various entities with value 0, I assume it stands
// for "no reference". Just in case, consider all values which round to that as "no reference" as
// well
pub(super) fn attr_val_to_fk(val: ed::EAttrVal) -> Option<KeyPart> {
    match val.round() as KeyPart {
        0 => None,
        v => Some(v),
    }
}
