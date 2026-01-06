use crate::{ad::generator::rels::KeyPart, ed::EFloat};

// Since CCP data is full of dead references to various entities with value 0, I assume it stands
// for "no reference". Just in case, consider all values which round to that as "no reference" as
// well
pub(super) fn attr_val_to_fk(val: EFloat) -> Option<KeyPart> {
    let fk = KeyPart::from_f64_rounded(val.into_f64());
    if fk == KeyPart::from_i32(0) {
        return None;
    }
    Some(fk)
}
