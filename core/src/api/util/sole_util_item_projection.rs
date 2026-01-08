use super::is_effect_projectable;
use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UItemId, UProjData},
};

impl SolarSystem {
    pub(in crate::api) fn util_add_item_projection(
        u_data: &UData,
        svc: &mut Svc,
        projector_uid: UItemId,
        projectee_uid: UItemId,
        proj_data: Option<UProjData>,
    ) {
        let projector_u_item = u_data.items.get(projector_uid);
        let projectee_u_item = u_data.items.get(projectee_uid);
        svc.notify_item_projected();
        if let Some(reffs) = projector_u_item.get_reffs() {
            for &effect_rid in reffs.iter() {
                let r_effect = u_data.src.get_effect_by_rid(effect_rid);
                if is_effect_projectable(projector_u_item, r_effect) {
                    svc.notify_effect_projected(
                        u_data,
                        projector_uid,
                        projector_u_item,
                        r_effect,
                        projectee_uid,
                        projectee_u_item,
                        proj_data,
                    );
                }
            }
        }
    }
    pub(in crate::api) fn util_remove_item_projection(
        u_data: &UData,
        svc: &mut Svc,
        projector_uid: UItemId,
        projectee_uid: UItemId,
    ) {
        let projector_u_item = u_data.items.get(projector_uid);
        let projectee_u_item = u_data.items.get(projectee_uid);
        if let Some(reffs) = projector_u_item.get_reffs() {
            for &effect_rid in reffs.iter() {
                let r_effect = u_data.src.get_effect_by_rid(effect_rid);
                if is_effect_projectable(projector_u_item, r_effect) {
                    svc.notify_effect_unprojected(
                        u_data,
                        projector_uid,
                        projector_u_item,
                        r_effect,
                        projectee_uid,
                        projectee_u_item,
                    );
                }
            }
        }
        svc.notify_item_unprojected();
    }
    pub(in crate::api) fn util_change_item_proj_data(
        u_data: &UData,
        svc: &mut Svc,
        projector_uid: UItemId,
        projectee_uid: UItemId,
        proj_data: Option<UProjData>,
    ) {
        let projector_u_item = u_data.items.get(projector_uid);
        let projectee_u_item = u_data.items.get(projectee_uid);
        svc.notify_item_proj_data_changed();
        if let Some(reffs) = projector_u_item.get_reffs() {
            for &effect_rid in reffs.iter() {
                let r_effect = u_data.src.get_effect_by_rid(effect_rid);
                if is_effect_projectable(projector_u_item, r_effect) {
                    svc.notify_effect_proj_data_changed(
                        u_data,
                        projector_uid,
                        r_effect.rid,
                        projectee_uid,
                        projectee_u_item,
                        proj_data,
                    );
                }
            }
        }
    }
}
