use crate::{
    ed::{EEffectId, EItemId},
    util::LibNamed,
};

pub struct EItemEffect {
    pub item_id: EItemId,
    pub effect_id: EEffectId,
    pub is_default: bool,
}
impl LibNamed for EItemEffect {
    fn lib_get_name() -> &'static str {
        "EItemEffect"
    }
}
