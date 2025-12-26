use crate::{
    ac,
    misc::{AttrSpec, EffectSpec},
    rd::REffect,
    svc::{eff_funcs, vast::Vast},
    ud::{UItem, UItemKey},
};

impl Vast {
    pub(in crate::svc) fn effect_projected(
        &mut self,
        projector_key: UItemKey,
        projector_item: &UItem,
        effect: &REffect,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        if let Some(projector_fit_key) = projector_item.get_fit_key() {
            let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
            let projector_espec = EffectSpec::new(projector_key, effect.key);
            if effect.projectee_filter.is_some()
                && let Some(effect_data) = projector_item.get_effect_datas().unwrap().get(&effect.key)
                && let Some(item_list_key) = effect_data.projectee_filter
            {
                projector_fit_data
                    .projectee_filter
                    .add_entry(projector_espec, projectee_key, item_list_key);
            }
            if effect.category == ac::effcats::TARGET {
                if effect.is_assist {
                    projector_fit_data
                        .blockable_assistance
                        .add_entry(projectee_key, projector_espec);
                }
                if is_offense_blockable(projector_item, effect) {
                    projector_fit_data
                        .blockable_offense
                        .add_entry(projectee_key, projector_espec);
                }
                if let Some(resist_attr_key) = eff_funcs::get_resist_attr_key(projector_item, effect) {
                    let projectee_aspec = AttrSpec::new(projectee_key, resist_attr_key);
                    projector_fit_data
                        .resist_immunity
                        .add_entry(projectee_aspec, projector_espec);
                }
            }
        }
        if let Some(projectee_fit_key) = projectee_item.get_fit_key()
            && !effect.stopped_effect_keys.is_empty()
            && effect.category == ac::effcats::TARGET
        {
            let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_key).unwrap();
            let stopper = EffectSpec::new(projector_key, effect.key);
            for stop_effect_key in effect.stopped_effect_keys.iter() {
                let stopped = EffectSpec::new(projectee_key, *stop_effect_key);
                projectee_fit_data.stopped_effects.add_entry(stopped, stopper);
            }
        }
        if let Some(rep_ospec) = effect.outgoing_shield_rep_opc_spec {
            if effect.is_active_with_duration {
                self.irr_shield
                    .add_entry(projectee_key, projector_key, effect.key, rep_ospec);
            }
            if effect.charge.is_some() {
                self.irr_shield_limitable
                    .add_entry(projectee_key, projector_key, effect.key, rep_ospec);
            }
        }
        if let Some(rep_ospec) = effect.outgoing_armor_rep_opc_spec {
            if effect.is_active_with_duration {
                self.irr_armor
                    .add_entry(projectee_key, projector_key, effect.key, rep_ospec);
            }
            if effect.charge.is_some() {
                self.irr_armor_limitable
                    .add_entry(projectee_key, projector_key, effect.key, rep_ospec);
            }
        }
        if let Some(rep_ospec) = effect.outgoing_hull_rep_opc_spec
            && effect.is_active_with_duration
        {
            self.irr_hull
                .add_entry(projectee_key, projector_key, effect.key, rep_ospec);
        }
        if let Some(rep_ospec) = effect.outgoing_cap_opc_spec
            && effect.is_active_with_duration
        {
            self.in_cap
                .add_entry(projectee_key, projector_key, effect.key, rep_ospec);
        }
        if let Some(neut_ospec) = effect.neut_opc_spec {
            self.in_neuts
                .add_entry(projectee_key, projector_key, effect.key, neut_ospec);
        }
        if let Some(ecm_ospec) = effect.ecm_opc_spec {
            self.in_ecm
                .add_entry(projectee_key, projector_key, effect.key, ecm_ospec);
        }
    }
    pub(in crate::svc) fn effect_unprojected(
        &mut self,
        projector_key: UItemKey,
        projector_item: &UItem,
        effect: &REffect,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        if let Some(projector_fit_key) = projector_item.get_fit_key() {
            let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
            let projector_espec = EffectSpec::new(projector_key, effect.key);
            if effect.projectee_filter.is_some() {
                projector_fit_data
                    .projectee_filter
                    .remove_l2(projector_espec, &projectee_key);
            }
            if effect.category == ac::effcats::TARGET {
                if effect.is_assist {
                    projector_fit_data
                        .blockable_assistance
                        .remove_entry(projectee_key, &projector_espec);
                }
                if is_offense_blockable(projector_item, effect) {
                    projector_fit_data
                        .blockable_offense
                        .remove_entry(projectee_key, &projector_espec);
                }
                if let Some(resist_attr_key) = eff_funcs::get_resist_attr_key(projector_item, effect) {
                    let projectee_aspec = AttrSpec::new(projectee_key, resist_attr_key);
                    projector_fit_data
                        .resist_immunity
                        .remove_entry(projectee_aspec, &projector_espec);
                }
            }
        }
        if let Some(projectee_fit_key) = projectee_item.get_fit_key()
            && !effect.stopped_effect_keys.is_empty()
            && effect.category == ac::effcats::TARGET
        {
            let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_key).unwrap();
            let stopper = EffectSpec::new(projector_key, effect.key);
            for stop_effect_key in effect.stopped_effect_keys.iter() {
                let stopped = EffectSpec::new(projectee_key, *stop_effect_key);
                projectee_fit_data.stopped_effects.remove_entry(stopped, &stopper);
            }
        }
        if effect.outgoing_shield_rep_opc_spec.is_some() {
            if effect.is_active_with_duration {
                self.irr_shield.remove_l3(projectee_key, projector_key, &effect.key);
            }
            if effect.charge.is_some() {
                self.irr_shield_limitable
                    .remove_l3(projectee_key, projector_key, &effect.key);
            }
        }
        if effect.outgoing_armor_rep_opc_spec.is_some() {
            if effect.is_active_with_duration {
                self.irr_armor.remove_l3(projectee_key, projector_key, &effect.key);
            }
            if effect.charge.is_some() {
                self.irr_armor_limitable
                    .remove_l3(projectee_key, projector_key, &effect.key);
            }
        }
        if effect.outgoing_hull_rep_opc_spec.is_some() && effect.is_active_with_duration {
            self.irr_hull.remove_l3(projectee_key, projector_key, &effect.key);
        }
        if effect.outgoing_cap_opc_spec.is_some() && effect.is_active_with_duration {
            self.in_cap.remove_l3(projectee_key, projector_key, &effect.key);
        }
        if effect.neut_opc_spec.is_some() {
            self.in_neuts.remove_l3(projectee_key, projector_key, &effect.key);
        }
        if effect.ecm_opc_spec.is_some() {
            self.in_ecm.remove_l3(projectee_key, projector_key, &effect.key);
        }
    }
}

fn is_offense_blockable(projector_item: &UItem, r_effect: &REffect) -> bool {
    if r_effect.is_offense && !r_effect.modifiers.is_empty() && !r_effect.ignore_offmod_immunity {
        return true;
    };
    // Assistance with extra flag can be blocked by the disallow offensive modifiers flag too
    r_effect.is_assist && projector_item.get_axt().unwrap().disallow_vs_ew_immune_tgt
}
