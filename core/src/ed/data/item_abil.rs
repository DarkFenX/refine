use crate::{
    ed::{EAbilId, EAttrVal, ECount, EItemId, ESlot},
    util::Named,
};

pub struct EItemFighterAbil {
    pub item_id: EItemId,
    pub abil_id: EAbilId,
    pub slot: ESlot,
    pub cooldown: Option<EAttrVal>,
    pub charge_count: Option<ECount>,
    pub charge_rearm_time: Option<EAttrVal>,
}
impl Named for EItemFighterAbil {
    fn get_name() -> &'static str {
        "EItemFighterAbil"
    }
}
