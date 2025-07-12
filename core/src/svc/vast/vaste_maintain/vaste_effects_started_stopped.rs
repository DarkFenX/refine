use crate::{
    ad,
    def::{FitKey, ItemKey},
    misc::EffectSpec,
    svc::{efuncs, vast::Vast},
    uad::UadItem,
};

impl Vast {
    pub(in crate::svc) fn effects_started(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        match item {
            UadItem::Drone(drone) => {
                for a_effect in a_effects {
                    self.handle_orrs_start(a_effect, item_key, &drone.get_fit_key());
                }
            }
            UadItem::Fighter(fighter) => {
                for a_effect in a_effects {
                    self.handle_orrs_start(a_effect, item_key, &fighter.get_fit_key());
                }
            }
            UadItem::Module(module) => {
                for a_effect in a_effects {
                    // Local reps
                    if let Some(rep_getter) = a_effect.hc.get_local_shield_rep_amount {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                        if efuncs::has_cycle_time(a_effect) {
                            fit_data
                                .lr_shield
                                .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                        }
                        if a_effect.hc.charge.is_some() {
                            fit_data
                                .lr_shield_limitable
                                .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                        }
                    }
                    if let Some(rep_getter) = a_effect.hc.get_local_armor_rep_amount {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                        if efuncs::has_cycle_time(a_effect) {
                            fit_data
                                .lr_armor
                                .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                        }
                        if a_effect.hc.charge.is_some() {
                            fit_data
                                .lr_armor_limitable
                                .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                        }
                    }
                    if let Some(rep_getter) = a_effect.hc.get_local_hull_rep_amount
                        && efuncs::has_cycle_time(a_effect)
                    {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                        fit_data
                            .lr_hull
                            .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                    }
                    // Remote reps
                    self.handle_orrs_start(a_effect, item_key, &module.get_fit_key());
                }
            }
            _ => (),
        }
        for a_effect in a_effects {
            if let Some(fit_key) = item.get_fit_key() {
                // Remote reps
                if let Some(rep_getter) = a_effect.hc.get_remote_shield_rep_amount
                    && efuncs::has_cycle_time(a_effect)
                {
                    let fit_data = self.get_fit_data_mut(&fit_key);
                    fit_data
                        .orr_shield
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
                if let Some(rep_getter) = a_effect.hc.get_remote_armor_rep_amount
                    && efuncs::has_cycle_time(a_effect)
                {
                    let fit_data = self.get_fit_data_mut(&fit_key);
                    fit_data
                        .orr_armor
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
                if let Some(rep_getter) = a_effect.hc.get_remote_hull_rep_amount
                    && efuncs::has_cycle_time(a_effect)
                {
                    let fit_data = self.get_fit_data_mut(&fit_key);
                    fit_data
                        .orr_hull
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
                if let Some(rep_getter) = a_effect.hc.get_remote_cap_rep_amount
                    && efuncs::has_cycle_time(a_effect)
                {
                    let fit_data = self.get_fit_data_mut(&fit_key);
                    fit_data
                        .orr_cap
                        .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
                }
            }
        }
    }
    pub(in crate::svc) fn effects_stopped(&mut self, item_key: ItemKey, item: &UadItem, a_effects: &[ad::ArcEffectRt]) {
        match item {
            UadItem::Drone(drone) => {
                for a_effect in a_effects {
                    self.handle_orrs_stop(a_effect, item_key, &drone.get_fit_key());
                }
            }
            UadItem::Fighter(fighter) => {
                for a_effect in a_effects {
                    self.handle_orrs_stop(a_effect, item_key, &fighter.get_fit_key());
                }
            }
            UadItem::Module(module) => {
                for a_effect in a_effects {
                    // Local reps
                    if a_effect.hc.get_local_shield_rep_amount.is_some() {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                        if efuncs::has_cycle_time(a_effect) {
                            fit_data.lr_shield.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                        }
                        if a_effect.hc.charge.is_some() {
                            fit_data
                                .lr_shield_limitable
                                .remove(&EffectSpec::new(item_key, a_effect.ae.id));
                        }
                    }
                    if a_effect.hc.get_local_armor_rep_amount.is_some() {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                        if efuncs::has_cycle_time(a_effect) {
                            fit_data.lr_armor.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                        }
                        if a_effect.hc.charge.is_some() {
                            fit_data
                                .lr_armor_limitable
                                .remove(&EffectSpec::new(item_key, a_effect.ae.id));
                        }
                    }
                    if a_effect.hc.get_local_hull_rep_amount.is_some() && efuncs::has_cycle_time(a_effect) {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                        fit_data.lr_hull.remove(&EffectSpec::new(item_key, a_effect.ae.id));
                    }
                    // Remote reps
                    self.handle_orrs_stop(a_effect, item_key, &module.get_fit_key());
                }
            }
            _ => (),
        }
    }
    fn handle_orrs_start(&mut self, a_effect: &ad::ArcEffectRt, item_key: ItemKey, fit_key: &FitKey) {
        if let Some(rep_getter) = a_effect.hc.get_remote_shield_rep_amount
            && efuncs::has_cycle_time(a_effect)
        {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data
                .orr_shield
                .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
        }
        if let Some(rep_getter) = a_effect.hc.get_remote_armor_rep_amount
            && efuncs::has_cycle_time(a_effect)
        {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data
                .orr_armor
                .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
        }
        if let Some(rep_getter) = a_effect.hc.get_remote_hull_rep_amount
            && efuncs::has_cycle_time(a_effect)
        {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data
                .orr_hull
                .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
        }
        if let Some(rep_getter) = a_effect.hc.get_remote_cap_rep_amount
            && efuncs::has_cycle_time(a_effect)
        {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data
                .orr_cap
                .insert(EffectSpec::new(item_key, a_effect.ae.id), rep_getter);
        }
    }
    fn handle_orrs_stop(&mut self, a_effect: &ad::ArcEffectRt, item_key: ItemKey, fit_key: &FitKey) {
        if a_effect.hc.get_remote_shield_rep_amount.is_some() && efuncs::has_cycle_time(a_effect) {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_shield.remove(&EffectSpec::new(item_key, a_effect.ae.id));
        }
        if a_effect.hc.get_remote_armor_rep_amount.is_some() && efuncs::has_cycle_time(a_effect) {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_armor.remove(&EffectSpec::new(item_key, a_effect.ae.id));
        }
        if a_effect.hc.get_remote_hull_rep_amount.is_some() && efuncs::has_cycle_time(a_effect) {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_hull.remove(&EffectSpec::new(item_key, a_effect.ae.id));
        }
        if a_effect.hc.get_remote_cap_rep_amount.is_some() && efuncs::has_cycle_time(a_effect) {
            let fit_data = self.get_fit_data_mut(fit_key);
            fit_data.orr_cap.remove(&EffectSpec::new(item_key, a_effect.ae.id));
        }
    }
}
