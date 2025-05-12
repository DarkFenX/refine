use ordered_float::OrderedFloat as OF;

use crate::{
    ac, ad,
    sol::{
        ItemKey,
        svc::{EffectSpec, vast::Vast},
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
        if !a_effect.stop_ids.is_empty() {
            if let Some(projectee_fit_key) = projectee_item.get_fit_key() {
                let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_key).unwrap();
                let stopper = EffectSpec {
                    item_key: projector_item_key,
                    a_effect_id: a_effect.id,
                };
                for stop_a_effect_id in a_effect.stop_ids.iter() {
                    let stopped = EffectSpec {
                        item_key: projectee_item_key,
                        a_effect_id: *stop_a_effect_id,
                    };
                    projectee_fit_data.stopped_effects.add_entry(stopped, stopper);
                }
            }
        }
        if is_offense_blockable(projector_item, a_effect) {
            if let Some(projector_fit_key) = projector_item.get_fit_key() {
                let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
                let projector_spec = EffectSpec {
                    item_key: projector_item_key,
                    a_effect_id: a_effect.id,
                };
                projector_fit_data
                    .blockable_offense
                    .add_entry(projector_spec, projectee_item_key);
            }
        }
        if a_effect.is_assist {
            if let Some(projector_fit_key) = projector_item.get_fit_key() {
                let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
                let projector_spec = EffectSpec {
                    item_key: projector_item_key,
                    a_effect_id: a_effect.id,
                };
                projector_fit_data
                    .blockable_assistance
                    .add_entry(projector_spec, projectee_item_key);
            }
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
        if !a_effect.stop_ids.is_empty() {
            if let Some(projectee_fit_key) = projectee_item.get_fit_key() {
                let projectee_fit_data = self.fit_datas.get_mut(&projectee_fit_key).unwrap();
                let stopper = EffectSpec {
                    item_key: projector_item_key,
                    a_effect_id: a_effect.id,
                };
                for stop_a_effect_id in a_effect.stop_ids.iter() {
                    let stopped = EffectSpec {
                        item_key: projectee_item_key,
                        a_effect_id: *stop_a_effect_id,
                    };
                    projectee_fit_data.stopped_effects.remove_entry(&stopped, &stopper);
                }
            }
        }
        if is_offense_blockable(projector_item, a_effect) {
            if let Some(projector_fit_key) = projector_item.get_fit_key() {
                let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
                let projector_spec = EffectSpec {
                    item_key: projector_item_key,
                    a_effect_id: a_effect.id,
                };
                projector_fit_data
                    .blockable_offense
                    .remove_entry(&projector_spec, &projectee_item_key);
            }
        }
        if a_effect.is_assist {
            if let Some(projector_fit_key) = projector_item.get_fit_key() {
                let projector_fit_data = self.fit_datas.get_mut(&projector_fit_key).unwrap();
                let projector_spec = EffectSpec {
                    item_key: projector_item_key,
                    a_effect_id: a_effect.id,
                };
                projector_fit_data
                    .blockable_assistance
                    .remove_entry(&projector_spec, &projectee_item_key);
            }
        }
    }
}

fn is_offense_blockable(projector_item: &UadItem, a_effect: &ad::ArcEffect) -> bool {
    if a_effect.is_offense && !a_effect.mods.is_empty() {
        return true;
    };
    // Assistance with extra flag can be blocked by the disallow offensive modifiers flag too
    a_effect.is_assist
        && projector_item
            .get_a_attr(&ac::attrs::DISALLOW_VS_EW_IMMUNE_TGT)
            .unwrap_or(OF(0.0))
            != OF(0.0)
}
