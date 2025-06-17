use crate::{
    ac, ad,
    sol::{
        ItemKey,
        svc::{EffectSpec, vast::Vast},
        uad::item::UadItem,
    },
};

impl Vast {
    pub(in crate::sol::svc) fn effects_started(
        &mut self,
        item_key: ItemKey,
        item: &UadItem,
        a_effects: &[ad::ArcEffect],
    ) {
        for a_effect in a_effects {
            match a_effect.id {
                ac::effects::FUELED_SHIELD_BOOSTING => {
                    if let Some(fit_id) = item.get_fit_key() {
                        let fit_data = self.get_fit_data_mut(&fit_id);
                        fit_data.limitable_sr.insert(EffectSpec::new(item_key, a_effect.id));
                    }
                }
                ac::effects::FUELED_ARMOR_REPAIR => {
                    if let Some(fit_id) = item.get_fit_key() {
                        let fit_data = self.get_fit_data_mut(&fit_id);
                        fit_data.limitable_ar.insert(EffectSpec::new(item_key, a_effect.id));
                    }
                }
                _ => (),
            }
        }
    }
    pub(in crate::sol::svc) fn effects_stopped(
        &mut self,
        item_key: ItemKey,
        item: &UadItem,
        a_effects: &[ad::ArcEffect],
    ) {
        for a_effect in a_effects {
            match a_effect.id {
                ac::effects::FUELED_SHIELD_BOOSTING => {
                    if let Some(fit_id) = item.get_fit_key() {
                        let fit_data = self.get_fit_data_mut(&fit_id);
                        fit_data.limitable_sr.insert(EffectSpec::new(item_key, a_effect.id));
                    }
                }
                ac::effects::FUELED_ARMOR_REPAIR => {
                    if let Some(fit_id) = item.get_fit_key() {
                        let fit_data = self.get_fit_data_mut(&fit_id);
                        fit_data.limitable_ar.insert(EffectSpec::new(item_key, a_effect.id));
                    }
                }
                _ => (),
            }
        }
    }
}
