use super::is_a_effect_projectable;
use crate::{
    def::ItemKey,
    sol::SolarSystem,
    svc::Svc,
    uad::{Uad, UadItem, UadProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_item_projection(
        uad: &Uad,
        svc: &mut Svc,
        projector_key: ItemKey,
        projector_uad_item: &UadItem,
        projectee_key: ItemKey,
        projectee_uad_item: &UadItem,
        range: Option<UadProjRange>,
    ) {
        svc.notify_item_projected();
        if let Some(reffs) = projector_uad_item.get_reffs() {
            for a_effect_id in reffs.iter() {
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
    }
    pub(in crate::sol::api) fn util_remove_item_projection(
        uad: &Uad,
        svc: &mut Svc,
        projector_key: ItemKey,
        projector_uad_item: &UadItem,
        projectee_key: ItemKey,
        projectee_uad_item: &UadItem,
    ) {
        if let Some(reffs) = projector_uad_item.get_reffs() {
            for a_effect_id in reffs.iter() {
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
        }
        svc.notify_item_unprojected();
    }
    pub(in crate::sol::api) fn util_change_item_proj_range(
        uad: &Uad,
        svc: &mut Svc,
        projector_key: ItemKey,
        projector_uad_item: &UadItem,
        projectee_key: ItemKey,
        projectee_uad_item: &UadItem,
        range: Option<UadProjRange>,
    ) {
        svc.notify_item_proj_range_changed();
        if let Some(reffs) = projector_uad_item.get_reffs() {
            for a_effect_id in reffs.iter() {
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
