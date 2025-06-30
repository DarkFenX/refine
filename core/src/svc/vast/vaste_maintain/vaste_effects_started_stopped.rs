use crate::{ac, ad, def::ItemKey, misc::EffectSpec, svc::vast::Vast, uad::UadItem};

impl Vast {
    pub(in crate::svc) fn effects_started(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        for a_effect in a_effects {
            match a_effect.ae.id {
                ac::effects::FUELED_SHIELD_BOOSTING => {
                    if let Some(fit_id) = item.get_fit_key() {
                        let fit_data = self.get_fit_data_mut(&fit_id);
                        fit_data.limitable_sb.insert(EffectSpec::new(item_key, a_effect.ae.id));
                    }
                }
                ac::effects::FUELED_ARMOR_REPAIR => {
                    if let Some(fit_id) = item.get_fit_key() {
                        let fit_data = self.get_fit_data_mut(&fit_id);
                        fit_data.limitable_ar.insert(EffectSpec::new(item_key, a_effect.ae.id));
                    }
                }
                _ => (),
            }
        }
    }
    pub(in crate::svc) fn effects_stopped(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        for a_effect in a_effects {
            match a_effect.ae.id {
                ac::effects::FUELED_SHIELD_BOOSTING => {
                    if let Some(fit_id) = item.get_fit_key() {
                        let fit_data = self.get_fit_data_mut(&fit_id);
                        fit_data.limitable_sb.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                    }
                }
                ac::effects::FUELED_ARMOR_REPAIR => {
                    if let Some(fit_id) = item.get_fit_key() {
                        let fit_data = self.get_fit_data_mut(&fit_id);
                        fit_data.limitable_ar.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                    }
                }
                _ => (),
            }
        }
    }
}
