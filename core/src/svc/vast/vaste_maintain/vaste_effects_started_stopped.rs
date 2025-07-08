use crate::{ad, def::ItemKey, misc::EffectSpec, svc::vast::Vast, uad::UadItem};

impl Vast {
    pub(in crate::svc) fn effects_started(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        for a_effect in a_effects {
            if let Some(fit_id) = item.get_fit_key() {
                if let Some(rep_getter) = a_effect.hc.get_local_shield_rep_amount
                    && a_effect.hc.charge.is_some()
                {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .limitable_sb
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
                if let Some(rep_getter) = a_effect.hc.get_local_armor_rep_amount
                    && a_effect.hc.charge.is_some()
                {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .limitable_ar
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
            }
        }
    }
    pub(in crate::svc) fn effects_stopped(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        for a_effect in a_effects {
            if let Some(fit_id) = item.get_fit_key() {
                if a_effect.hc.get_local_shield_rep_amount.is_some() && a_effect.hc.charge.is_some() {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data.limitable_sb.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                }
                if a_effect.hc.get_local_armor_rep_amount.is_some() && a_effect.hc.charge.is_some() {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data.limitable_ar.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                }
            }
        }
    }
}
