use crate::ad::{ACount, AItemId, AItemListId, AValue};

#[derive(Clone, Default)]
pub struct AItemEffectData {
    pub autocharge: Option<AItemId> = None,
    pub cooldown: Option<AValue> = None,
    pub charge_count: Option<ACount> = None,
    pub charge_reload_time: Option<AValue> = None,
    pub projectee_filter: Option<AItemListId> = None,
}
