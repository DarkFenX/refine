use crate::{
    ed::{EEffectId, EItemId},
    util::Named,
};

pub struct EItemEffect {
    pub item_id: EItemId,
    pub effect_id: EEffectId,
    pub is_default: bool,
}
impl Named for EItemEffect {
    fn get_name() -> &'static str {
        "EItemEffect"
    }
}
