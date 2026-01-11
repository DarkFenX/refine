use crate::ad::{ACount, AItemId, AItemListId, AValue};

#[derive(Copy, Clone)]
pub struct AItemEffectData {
    pub autocharge: Option<AItemId>,
    pub cooldown: Option<AValue>,
    pub charge_count: Option<ACount>,
    pub charge_reload_time: Option<AValue>,
    pub projectee_filter: Option<AItemListId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemEffectData {
    pub(in crate::ad::data::item) const fn default() -> Self {
        AItemEffectData {
            autocharge: None,
            cooldown: None,
            charge_count: None,
            charge_reload_time: None,
            projectee_filter: None,
        }
    }
}
