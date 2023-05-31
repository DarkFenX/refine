use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct ChargeInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub cont_id: ReeId,
}
impl ChargeInfo {
    fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt, cont_id: ReeId) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            cont_id,
        }
    }
}
impl From<&ssi::Charge> for ChargeInfo {
    fn from(c: &ssi::Charge) -> Self {
        ChargeInfo::new(c.id, c.fit_id, c.type_id, c.cont_id)
    }
}
