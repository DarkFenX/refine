use crate::{
    ad,
    sol::{ItemKey, svc::Svc},
};

impl Svc {
    pub(in crate::sol) fn is_effect_running(&self, item_key: &ItemKey, a_effect_id: &ad::AEffectId) -> bool {
        self.running_effects.is_running(item_key, a_effect_id)
    }
}
