use crate::ad::{AAttrVal, ACount, AItemId, AItemListId};

#[derive(Clone, Default)]
pub struct AItemEffectData {
    pub autocharge: Option<AItemId> = None,
    pub cooldown: Option<AAttrVal> = None,
    pub charge_count: Option<ACount> = None,
    pub charge_reload_time: Option<AAttrVal> = None,
    pub projectee_filter: Option<AItemListId> = None,
}
