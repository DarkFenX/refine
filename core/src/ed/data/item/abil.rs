use crate::{
    ed::{EAbilId, EGenFloat, EGenInt, EItemId},
    util::LibNamed,
};

pub struct EItemAbil {
    pub item_id: EItemId,
    pub abil_id: EAbilId,
    pub slot: EGenInt,
    pub cooldown: Option<EGenFloat>,
    pub charge_count: Option<EGenInt>,
    pub charge_rearm_time: Option<EGenFloat>,
}
impl LibNamed for EItemAbil {
    fn lib_get_name() -> &'static str {
        "EItemAbil"
    }
}
