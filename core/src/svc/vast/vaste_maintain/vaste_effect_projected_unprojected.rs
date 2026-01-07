use crate::{
    ad::AEffectCatId,
    misc::{AttrSpec, EffectSpec},
    rd::REffect,
    svc::{funcs, vast::Vast},
    ud::{UItem, UItemId},
};

impl Vast {
    pub(in crate::svc) fn effect_projected(
        &mut self,
        projector_uid: UItemId,
        projector_item: &UItem,
        effect: &REffect,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        if let Some(projector_fit_uid) = projector_item.get_fit_uid() {
            let projector_fit_data = self.fit_datas.get_mut(&projector_fit_uid).unwrap();
            let projector_espec = EffectSpec::new(projector_uid, effect.rid);
            if effect.projectee_filter.is_some()
                && let Some(effect_data) = projector_item.get_effect_datas().unwrap().get(&effect.rid)
                && let Some(item_list_rid) = effect_data.projectee_filter
            {
                projector_fit_data
                    .projectee_filter
                    .add_entry(projector_espec, projectee_uid, item_list_rid);
            }
            if effect.category == AEffectCatId::TARGET {
                if effect.is_assist {
                    projector_fit_data
                        .blockable_assistance
                        .add_entry(projectee_uid, projector_espec);
                }
                if is_offense_blockable(projector_item, effect) {
                    projector_fit_data
                        .blockable_offense
                        .add_entry(projectee_uid, projector_espec);
                }
                if let Some(resist_attr_rid) = funcs::get_resist_attr_rid(projector_item, effect) {
                    let projectee_aspec = AttrSpec::new(projectee_uid, resist_attr_rid);
                    projector_fit_data
                        .resist_immunity
                        .add_entry(projectee_aspec, projector_espec);
                }
            }
        }
        if let Some(projectee_fit_uid) = projectee_item.get_fit_uid()
            && !effect.stopped_effect_rids.is_empty()
            && effect.category == AEffectCatId::TARGET
        {
            let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_uid).unwrap();
            let stopper = EffectSpec::new(projector_uid, effect.rid);
            for stop_effect_rid in effect.stopped_effect_rids.iter() {
                let stopped = EffectSpec::new(projectee_uid, *stop_effect_rid);
                projectee_fit_data.stopped_effects.add_entry(stopped, stopper);
            }
        }
        if let Some(rep_ospec) = effect.outgoing_shield_rep_opc_spec {
            if effect.is_active_with_duration {
                self.irr_shield
                    .add_entry(projectee_uid, projector_uid, effect.rid, rep_ospec);
            }
            if effect.charge.is_some() {
                self.irr_shield_limitable
                    .add_entry(projectee_uid, projector_uid, effect.rid, rep_ospec);
            }
        }
        if let Some(rep_ospec) = effect.outgoing_armor_rep_opc_spec {
            if effect.is_active_with_duration {
                self.irr_armor
                    .add_entry(projectee_uid, projector_uid, effect.rid, rep_ospec);
            }
            if effect.charge.is_some() {
                self.irr_armor_limitable
                    .add_entry(projectee_uid, projector_uid, effect.rid, rep_ospec);
            }
        }
        if let Some(rep_ospec) = effect.outgoing_hull_rep_opc_spec
            && effect.is_active_with_duration
        {
            self.irr_hull
                .add_entry(projectee_uid, projector_uid, effect.rid, rep_ospec);
        }
        if let Some(rep_ospec) = effect.outgoing_cap_opc_spec
            && effect.is_active_with_duration
        {
            self.in_cap
                .add_entry(projectee_uid, projector_uid, effect.rid, rep_ospec);
        }
        if let Some(neut_ospec) = effect.neut_opc_spec {
            self.in_neuts
                .add_entry(projectee_uid, projector_uid, effect.rid, neut_ospec);
        }
        if let Some(ecm_ospec) = effect.ecm_opc_spec {
            self.in_ecm
                .add_entry(projectee_uid, projector_uid, effect.rid, ecm_ospec);
        }
    }
    pub(in crate::svc) fn effect_unprojected(
        &mut self,
        projector_uid: UItemId,
        projector_item: &UItem,
        effect: &REffect,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        if let Some(projector_fit_uid) = projector_item.get_fit_uid() {
            let projector_fit_data = self.fit_datas.get_mut(&projector_fit_uid).unwrap();
            let projector_espec = EffectSpec::new(projector_uid, effect.rid);
            if effect.projectee_filter.is_some() {
                projector_fit_data
                    .projectee_filter
                    .remove_l2(projector_espec, &projectee_uid);
            }
            if effect.category == AEffectCatId::TARGET {
                if effect.is_assist {
                    projector_fit_data
                        .blockable_assistance
                        .remove_entry(projectee_uid, &projector_espec);
                }
                if is_offense_blockable(projector_item, effect) {
                    projector_fit_data
                        .blockable_offense
                        .remove_entry(projectee_uid, &projector_espec);
                }
                if let Some(resist_attr_rid) = funcs::get_resist_attr_rid(projector_item, effect) {
                    let projectee_aspec = AttrSpec::new(projectee_uid, resist_attr_rid);
                    projector_fit_data
                        .resist_immunity
                        .remove_entry(projectee_aspec, &projector_espec);
                }
            }
        }
        if let Some(projectee_fit_uid) = projectee_item.get_fit_uid()
            && !effect.stopped_effect_rids.is_empty()
            && effect.category == AEffectCatId::TARGET
        {
            let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_uid).unwrap();
            let stopper = EffectSpec::new(projector_uid, effect.rid);
            for stop_effect_rid in effect.stopped_effect_rids.iter() {
                let stopped = EffectSpec::new(projectee_uid, *stop_effect_rid);
                projectee_fit_data.stopped_effects.remove_entry(stopped, &stopper);
            }
        }
        if effect.outgoing_shield_rep_opc_spec.is_some() {
            if effect.is_active_with_duration {
                self.irr_shield.remove_l3(projectee_uid, projector_uid, &effect.rid);
            }
            if effect.charge.is_some() {
                self.irr_shield_limitable
                    .remove_l3(projectee_uid, projector_uid, &effect.rid);
            }
        }
        if effect.outgoing_armor_rep_opc_spec.is_some() {
            if effect.is_active_with_duration {
                self.irr_armor.remove_l3(projectee_uid, projector_uid, &effect.rid);
            }
            if effect.charge.is_some() {
                self.irr_armor_limitable
                    .remove_l3(projectee_uid, projector_uid, &effect.rid);
            }
        }
        if effect.outgoing_hull_rep_opc_spec.is_some() && effect.is_active_with_duration {
            self.irr_hull.remove_l3(projectee_uid, projector_uid, &effect.rid);
        }
        if effect.outgoing_cap_opc_spec.is_some() && effect.is_active_with_duration {
            self.in_cap.remove_l3(projectee_uid, projector_uid, &effect.rid);
        }
        if effect.neut_opc_spec.is_some() {
            self.in_neuts.remove_l3(projectee_uid, projector_uid, &effect.rid);
        }
        if effect.ecm_opc_spec.is_some() {
            self.in_ecm.remove_l3(projectee_uid, projector_uid, &effect.rid);
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
