use super::is_a_effect_projectable;
use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UItem, UItemKey, UProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_item_projection(
        u_data: &UData,
        svc: &mut Svc,
        projector_key: UItemKey,
        projector_u_item: &UItem,
        projectee_key: UItemKey,
        projectee_u_item: &UItem,
        range: Option<UProjRange>,
    ) {
        svc.notify_item_projected();
        if let Some(reffs) = projector_u_item.get_reffs() {
            for a_effect_id in reffs.iter() {
                let r_effect = u_data.src.get_r_effect(a_effect_id).unwrap();
                if is_a_effect_projectable(projector_u_item, r_effect) {
                    svc.notify_effect_projected(
                        u_data,
                        projector_key,
                        projector_u_item,
                        r_effect,
                        projectee_key,
                        projectee_u_item,
                        range,
                    );
                }
            }
        }
    }
    pub(in crate::sol::api) fn util_remove_item_projection(
        u_data: &UData,
        svc: &mut Svc,
        projector_key: UItemKey,
        projector_u_item: &UItem,
        projectee_key: UItemKey,
        projectee_u_item: &UItem,
    ) {
        if let Some(reffs) = projector_u_item.get_reffs() {
            for a_effect_id in reffs.iter() {
                let r_effect = u_data.src.get_r_effect(a_effect_id).unwrap();
                if is_a_effect_projectable(projector_u_item, r_effect) {
                    svc.notify_effect_unprojected(
                        u_data,
                        projector_key,
                        projector_u_item,
                        r_effect,
                        projectee_key,
                        projectee_u_item,
                    );
                }
            }
        }
        svc.notify_item_unprojected();
    }
    pub(in crate::sol::api) fn util_change_item_proj_range(
        u_data: &UData,
        svc: &mut Svc,
        projector_key: UItemKey,
        projector_u_item: &UItem,
        projectee_key: UItemKey,
        projectee_u_item: &UItem,
        range: Option<UProjRange>,
    ) {
        svc.notify_item_proj_range_changed();
        if let Some(reffs) = projector_u_item.get_reffs() {
            for a_effect_id in reffs.iter() {
                let r_effect = u_data.src.get_r_effect(a_effect_id).unwrap();
                if is_a_effect_projectable(projector_u_item, r_effect) {
                    svc.notify_effect_proj_range_changed(
                        u_data,
                        projector_key,
                        r_effect.get_id(),
                        projectee_key,
                        projectee_u_item,
                        range,
                    );
                }
            }
        }
    }
}
