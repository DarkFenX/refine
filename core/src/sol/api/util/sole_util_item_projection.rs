use super::is_a_effect_projectable;
use crate::{
    def::ItemKey,
    sol::{SolarSystem, reffs::REffs},
    svc::Svc,
    uad::{Uad, UadItem, UadProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_item_projection(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &REffs,
        projector_key: ItemKey,
        projector_uad_item: &UadItem,
        projectee_key: ItemKey,
        projectee_uad_item: &UadItem,
        range: Option<UadProjRange>,
    ) {
        svc.notify_item_projected();
        for a_effect_id in reffs.iter_running(&projector_key) {
            let a_effect = uad.src.get_a_effect(a_effect_id).unwrap();
            if is_a_effect_projectable(projector_uad_item, a_effect) {
                svc.notify_effect_projected(
                    uad,
                    projector_key,
                    projector_uad_item,
                    a_effect,
                    projectee_key,
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
        projector_key: ItemKey,
        projector_uad_item: &UadItem,
        projectee_key: ItemKey,
        projectee_uad_item: &UadItem,
    ) {
        for a_effect_id in reffs.iter_running(&projector_key) {
            let effect = uad.src.get_a_effect(a_effect_id).unwrap();
            if is_a_effect_projectable(projector_uad_item, effect) {
                svc.notify_effect_unprojected(
                    uad,
                    projector_key,
                    projector_uad_item,
                    effect,
                    projectee_key,
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
        projector_key: ItemKey,
        projectee_key: ItemKey,
        projectee_uad_item: &UadItem,
        range: Option<UadProjRange>,
    ) {
        svc.notify_item_proj_range_changed();
        let running_a_effect_ids = reffs.iter_running(&projector_key);
        if !running_a_effect_ids.is_empty() {
            let projector_uad_item = uad.items.get(projector_key);
            for a_effect_id in reffs.iter_running(&projector_key) {
                let a_effect = uad.src.get_a_effect(a_effect_id).unwrap();
                if is_a_effect_projectable(projector_uad_item, a_effect) {
                    svc.notify_effect_proj_range_changed(
                        uad,
                        projector_key,
                        a_effect.ae.id,
                        projectee_key,
                        projectee_uad_item,
                        range,
                    );
                }
            }
        }
    }
}
