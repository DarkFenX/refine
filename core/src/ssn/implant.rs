use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct ImplantInfo {
    pub id: ReeId,
    pub fit_id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl ImplantInfo {
    fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            enabled,
        }
    }
}
impl From<&ssi::Implant> for ImplantInfo {
    fn from(i: &ssi::Implant) -> Self {
        ImplantInfo::new(i.id, i.fit_id, i.type_id, i.get_bool_state())
    }
}
