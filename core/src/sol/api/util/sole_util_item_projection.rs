use super::is_a_effect_projectable;
use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UItemKey, UProjData},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_item_projection(
        u_data: &UData,
        svc: &mut Svc,
        projector_key: UItemKey,
        projectee_key: UItemKey,
        proj_data: Option<UProjData>,
    ) {
        let projector_u_item = u_data.items.get(projector_key);
        let projectee_u_item = u_data.items.get(projectee_key);
        svc.notify_item_projected();
        if let Some(reffs) = projector_u_item.get_reffs() {
            for &effect_key in reffs.iter() {
                let r_effect = u_data.src.get_effect(effect_key);
                if is_a_effect_projectable(projector_u_item, r_effect) {
                    svc.notify_effect_projected(
                        u_data,
                        projector_key,
                        projector_u_item,
                        r_effect,
                        projectee_key,
                        projectee_u_item,
                        proj_data,
                    );
                }
            }
        }
    }
    pub(in crate::sol::api) fn util_remove_item_projection(
        u_data: &UData,
        svc: &mut Svc,
        projector_key: UItemKey,
        projectee_key: UItemKey,
    ) {
        let projector_u_item = u_data.items.get(projector_key);
        let projectee_u_item = u_data.items.get(projectee_key);
        if let Some(reffs) = projector_u_item.get_reffs() {
            for &effect_key in reffs.iter() {
                let r_effect = u_data.src.get_effect(effect_key);
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
    pub(in crate::sol::api) fn util_change_item_proj_data(
        u_data: &UData,
        svc: &mut Svc,
        projector_key: UItemKey,
        projectee_key: UItemKey,
        proj_data: Option<UProjData>,
    ) {
        let projector_u_item = u_data.items.get(projector_key);
        let projectee_u_item = u_data.items.get(projectee_key);
        svc.notify_item_proj_data_changed();
        if let Some(reffs) = projector_u_item.get_reffs() {
            for &effect_key in reffs.iter() {
                let r_effect = u_data.src.get_effect(effect_key);
                if is_a_effect_projectable(projector_u_item, r_effect) {
                    svc.notify_effect_proj_data_changed(
                        u_data,
                        projector_key,
                        r_effect.key,
                        projectee_key,
                        projectee_u_item,
                        proj_data,
                    );
                }
            }
        }
    }
}
