use crate::{
    ad,
    def::ItemKey,
    misc::EffectSpec,
    svc::{efuncs, vast::Vast},
    uad::UadItem,
};

impl Vast {
    pub(in crate::svc) fn effects_started(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        for a_effect in a_effects {
            if let Some(fit_id) = item.get_fit_key() {
                // Local reps
                if let Some(rep_getter) = a_effect.hc.get_local_shield_rep_amount
                    && a_effect.hc.charge.is_some()
                {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .lr_shield_limitable
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
                if let Some(rep_getter) = a_effect.hc.get_local_armor_rep_amount
                    && a_effect.hc.charge.is_some()
                {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .lr_armor_limitable
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
                // Remote reps
                if let Some(rep_getter) = a_effect.hc.get_remote_shield_rep_amount
                    && efuncs::has_cycle_time(a_effect)
                {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .orr_shield
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
                if let Some(rep_getter) = a_effect.hc.get_remote_armor_rep_amount
                    && efuncs::has_cycle_time(a_effect)
                {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .orr_armor
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
                if let Some(rep_getter) = a_effect.hc.get_remote_struct_rep_amount
                    && efuncs::has_cycle_time(a_effect)
                {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .orr_struct
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
                if let Some(rep_getter) = a_effect.hc.get_remote_cap_rep_amount
                    && efuncs::has_cycle_time(a_effect)
                {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .orr_cap
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
            }
        }
    }
    pub(in crate::svc) fn effects_stopped(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        for a_effect in a_effects {
            if let Some(fit_id) = item.get_fit_key() {
                // Local reps
                if a_effect.hc.get_local_shield_rep_amount.is_some() && a_effect.hc.charge.is_some() {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .lr_shield_limitable
                        .remove(&EffectSpec::new(item_key, a_effect.ae.id));
                }
                if a_effect.hc.get_local_armor_rep_amount.is_some() && a_effect.hc.charge.is_some() {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data
                        .lr_armor_limitable
                        .remove(&EffectSpec::new(item_key, a_effect.ae.id));
                }
                // Remote reps
                if a_effect.hc.get_remote_shield_rep_amount.is_some() && efuncs::has_cycle_time(a_effect) {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data.orr_shield.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                }
                if a_effect.hc.get_remote_armor_rep_amount.is_some() && efuncs::has_cycle_time(a_effect) {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data.orr_armor.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                }
                if a_effect.hc.get_remote_struct_rep_amount.is_some() && efuncs::has_cycle_time(a_effect) {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data.orr_struct.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                }
                if a_effect.hc.get_remote_cap_rep_amount.is_some() && efuncs::has_cycle_time(a_effect) {
                    let fit_data = self.get_fit_data_mut(&fit_id);
                    fit_data.orr_cap.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                }
            }
        }
    }
}
