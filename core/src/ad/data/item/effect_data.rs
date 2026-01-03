use crate::ad::{ACount, AItemId, AItemListId, ATimeVal};

#[derive(Clone, Default)]
pub struct AItemEffectData {
    pub autocharge: Option<AItemId> = None,
    pub cooldown: Option<ATimeVal> = None,
    pub charge_count: Option<ACount> = None,
    pub charge_reload_time: Option<ATimeVal> = None,
    pub projectee_filter: Option<AItemListId> = None,
}
