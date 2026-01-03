use crate::{ad::generator::rels::KeyPart, def::Id, ed::EGenFloat};

// Since CCP data is full of dead references to various entities with value 0, I assume it stands
// for "no reference". Just in case, consider all values which round to that as "no reference" as
// well
pub(super) fn attr_val_to_fk(val: EGenFloat) -> Option<KeyPart> {
    match val.into_inner().clamp(Id::MIN as f64, Id::MAX as f64).round() as Id {
        0 => None,
        v => Some(KeyPart::new(v)),
    }
}
