use crate::{
    defs::{ReeId, ReeInt},
    ssi,
};

pub struct SwEffectInfo {
    pub id: ReeId,
    pub type_id: ReeInt,
    pub enabled: bool,
}
impl SwEffectInfo {
    fn new(id: ReeId, type_id: ReeInt, enabled: bool) -> Self {
        Self { id, type_id, enabled }
    }
}
impl From<&ssi::SwEffect> for SwEffectInfo {
    fn from(e: &ssi::SwEffect) -> Self {
        SwEffectInfo::new(e.id, e.type_id, e.get_bool_state())
    }
}
