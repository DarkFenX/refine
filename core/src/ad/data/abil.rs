use crate::{
    ad::{AAbilId, AEffectId},
    util::Named,
};

pub struct AAbil {
    pub id: AAbilId,
    pub effect_id: AEffectId,
}
impl Named for AAbil {
    fn get_name() -> &'static str {
        "AAbil"
    }
}
