use crate::{
    ed::{EAbilId, EGenFloat, EGenInt, EItemId},
    util::Named,
};

pub struct EItemAbil {
    pub item_id: EItemId,
    pub abil_id: EAbilId,
    pub slot: EGenInt,
    pub cooldown: Option<EGenFloat>,
    pub charge_count: Option<EGenInt>,
    pub charge_rearm_time: Option<EGenFloat>,
}
impl Named for EItemAbil {
    fn get_name() -> &'static str {
        "EItemAbil"
    }
}
