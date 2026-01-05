use crate::{ad::generator::rels::KeyPart, ed::EGenFloat};

// Since CCP data is full of dead references to various entities with value 0, I assume it stands
// for "no reference". Just in case, consider all values which round to that as "no reference" as
// well
pub(super) fn attr_val_to_fk(val: EGenFloat) -> Option<KeyPart> {
    let fk = KeyPart::new_f64_rounded(val.into_inner());
    if fk == KeyPart::new(0) {
        return None;
    }
    Some(fk)
}
