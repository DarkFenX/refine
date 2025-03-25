use crate::{
    ad,
    sol::{ItemId, svc::Svc},
};

impl Svc {
    pub(in crate::sol) fn is_effect_running(&self, item_id: &ItemId, a_effect_id: &ad::AEffectId) -> bool {
        self.running_effects.is_running(item_id, a_effect_id)
    }
}
