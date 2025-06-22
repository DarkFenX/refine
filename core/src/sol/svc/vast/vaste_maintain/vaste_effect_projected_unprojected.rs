use crate::{
    ac, ad,
    sol::{
        ItemKey,
        svc::{AttrSpec, EffectSpec, get_resist_a_attr_id, vast::Vast},
        uad::item::UadItem,
    },
};

impl Vast {
    pub(in crate::sol::svc) fn effect_projected(
        &mut self,
        projector_item_key: ItemKey,
        projector_item: &UadItem,
        a_effect: &ad::ArcEffect,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        if a_effect.category == ac::effcats::TARGET {
            if !a_effect.stop_ids.is_empty()
                && let Some(projectee_fit_key) = projectee_item.get_fit_key()
            {
                let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_key).unwrap();
                let stopper = EffectSpec::new(projector_item_key, a_effect.id);
                for stop_a_effect_id in a_effect.stop_ids.iter() {
                    let stopped = EffectSpec::new(projectee_item_key, *stop_a_effect_id);
                    projectee_fit_data.stopped_effects.add_entry(stopped, stopper);
                }
            }
            if let Some(projector_fit_key) = projector_item.get_fit_key() {
                let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
                let projector_espec = EffectSpec::new(projector_item_key, a_effect.id);
                if a_effect.is_assist {
                    projector_fit_data
                        .blockable_assistance
                        .add_entry(projectee_item_key, projector_espec);
                }
                if is_offense_blockable(projector_item, a_effect) {
                    projector_fit_data
                        .blockable_offense
                        .add_entry(projectee_item_key, projector_espec);
                }
                if let Some(resist_a_attr_id) = get_resist_a_attr_id(projector_item, a_effect) {
                    let projectee_aspec = AttrSpec::new(projectee_item_key, resist_a_attr_id);
                    projector_fit_data
                        .resist_immunity
                        .add_entry(projectee_aspec, projector_espec);
                }
            }
        }
        match a_effect.id {
            ac::effects::SHIP_MODULE_RASB => {
                let projector_espec = EffectSpec::new(projector_item_key, a_effect.id);
                self.limitable_rsr.add_entry(projectee_item_key, projector_espec);
            }
            ac::effects::SHIP_MODULE_RAAR => {
                let projector_espec = EffectSpec::new(projector_item_key, a_effect.id);
                self.limitable_rar.add_entry(projectee_item_key, projector_espec);
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn effect_unprojected(
        &mut self,
        projector_item_key: ItemKey,
        projector_item: &UadItem,
        a_effect: &ad::ArcEffect,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        if a_effect.category == ac::effcats::TARGET {
            if !a_effect.stop_ids.is_empty()
                && let Some(projectee_fit_key) = projectee_item.get_fit_key()
            {
                let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_key).unwrap();
                let stopper = EffectSpec::new(projector_item_key, a_effect.id);
                for stop_a_effect_id in a_effect.stop_ids.iter() {
                    let stopped = EffectSpec::new(projectee_item_key, *stop_a_effect_id);
                    projectee_fit_data.stopped_effects.remove_entry(&stopped, &stopper);
                }
            }
            if let Some(projector_fit_key) = projector_item.get_fit_key() {
                let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
                let projector_espec = EffectSpec::new(projector_item_key, a_effect.id);
                if a_effect.is_assist {
                    projector_fit_data
                        .blockable_assistance
                        .remove_entry(&projectee_item_key, &projector_espec);
                }
                if is_offense_blockable(projector_item, a_effect) {
                    projector_fit_data
                        .blockable_offense
                        .remove_entry(&projectee_item_key, &projector_espec);
                }
                if let Some(resist_a_attr_id) = get_resist_a_attr_id(projector_item, a_effect) {
                    let projectee_aspec = AttrSpec::new(projectee_item_key, resist_a_attr_id);
                    projector_fit_data
                        .resist_immunity
                        .remove_entry(&projectee_aspec, &projector_espec);
                }
            }
        }
        match a_effect.id {
            ac::effects::SHIP_MODULE_RASB => {
                let projector_espec = EffectSpec::new(projector_item_key, a_effect.id);
                self.limitable_rsr.remove_entry(&projectee_item_key, &projector_espec);
            }
            ac::effects::SHIP_MODULE_RAAR => {
                let projector_espec = EffectSpec::new(projector_item_key, a_effect.id);
                self.limitable_rar.remove_entry(&projectee_item_key, &projector_espec);
            }
            _ => (),
        }
    }
}

fn is_offense_blockable(projector_item: &UadItem, a_effect: &ad::ArcEffect) -> bool {
    if a_effect.is_offense && !a_effect.mods.is_empty() {
        return true;
    };
    // Assistance with extra flag can be blocked by the disallow offensive modifiers flag too
    a_effect.is_assist && projector_item.get_a_extras().unwrap().disallow_vs_ew_immune_tgt
}
