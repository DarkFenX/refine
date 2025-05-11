use crate::{
    ad,
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
                    projectee_fit_data.stopped_effects.add_entry(stopper, stopped);
                }
            }
        }
    }
    pub(in crate::sol::svc) fn effect_unprojected(
        &mut self,
        projector_item_key: ItemKey,
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
                    projectee_fit_data.stopped_effects.remove_entry(&stopper, &stopped);
                }
            }
        }
    }
}
