use super::is_a_effect_projectable;
use crate::sol::{
    AttrVal, ItemKey, SolarSystem,
    reffs::REffs,
    svc::Svc,
    uad::{Uad, item::UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_item_projection(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &REffs,
        projector_item_key: ItemKey,
        projector_uad_item: &UadItem,
        projectee_item_key: ItemKey,
        projectee_uad_item: &UadItem,
        range: Option<AttrVal>,
    ) {
        svc.notify_item_projected();
        for a_effect_id in reffs.iter_running(&projector_item_key) {
            let a_effect = uad.src.get_a_effect(a_effect_id).unwrap();
            if is_a_effect_projectable(projector_uad_item, a_effect) {
                svc.notify_effect_projected(
                    uad,
                    projector_item_key,
                    projector_uad_item,
                    a_effect,
                    projectee_item_key,
                    projectee_uad_item,
                    range,
                );
            }
        }
    }
    pub(in crate::sol::api) fn util_remove_item_projection(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &REffs,
        projector_item_key: ItemKey,
        projector_uad_item: &UadItem,
        projectee_item_key: ItemKey,
        projectee_uad_item: &UadItem,
    ) {
        for a_effect_id in reffs.iter_running(&projector_item_key) {
            let effect = uad.src.get_a_effect(a_effect_id).unwrap();
            if is_a_effect_projectable(projector_uad_item, effect) {
                svc.notify_effect_unprojected(
                    uad,
                    projector_item_key,
                    projector_uad_item,
                    effect,
                    projectee_item_key,
                    projectee_uad_item,
                );
            }
        }
        svc.notify_item_unprojected();
    }
    pub(in crate::sol::api) fn util_change_item_proj_range(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &REffs,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
        projectee_uad_item: &UadItem,
        range: Option<AttrVal>,
    ) {
        svc.notify_item_proj_range_changed();
        let running_a_effect_ids = reffs.iter_running(&projector_item_key);
        if !running_a_effect_ids.is_empty() {
            let projector_uad_item = uad.items.get(projector_item_key);
            for a_effect_id in reffs.iter_running(&projector_item_key) {
                let a_effect = uad.src.get_a_effect(a_effect_id).unwrap();
                if is_a_effect_projectable(projector_uad_item, a_effect) {
                    svc.notify_effect_proj_range_changed(
                        uad,
                        projector_item_key,
                        a_effect.ae.id,
                        projectee_item_key,
                        projectee_uad_item,
                        range,
                    );
                }
            }
        }
    }
}
