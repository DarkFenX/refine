use crate::{
    ed::{EAbilId, EFloat, EInt, EItemId},
    util::LibNamed,
};

pub struct EItemAbil {
    pub item_id: EItemId,
    pub abil_id: EAbilId,
    pub slot: EInt,
    pub cooldown: Option<EFloat>,
    pub charge_count: Option<EInt>,
    pub charge_rearm_duration: Option<EFloat>,
}
impl LibNamed for EItemAbil {
    fn lib_get_name() -> &'static str {
        "EItemAbil"
    }
}
