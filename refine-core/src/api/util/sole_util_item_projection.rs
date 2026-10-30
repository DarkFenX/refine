use super::is_effect_projectable;
use crate::{
    sol::{RevProjs, SolarSystem},
    svc::Svc,
    ud::{UData, UItemId, UProjData},
};

impl SolarSystem {
    /// Register projections stored on the item.
    pub(in crate::api) fn util_reg_item_projs(
        u_data: &UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_uid: UItemId,
    ) {
        if let Some(projs) = u_data.items.get(item_uid).get_projs() {
            for (projectee_uid, proj_data) in projs.iter() {
                rev_projs.reg_projectee(item_uid, projectee_uid);
                SolarSystem::util_add_item_projection(u_data, svc, item_uid, projectee_uid, proj_data);
            }
        }
    }
    /// Unregister projections stored on the item, but do not remove them from the item itself.
    pub(in crate::api) fn util_unreg_item_projs(
        u_data: &UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_uid: UItemId,
    ) {
        if let Some(projs) = u_data.items.get(item_uid).get_projs() {
            for projectee_uid in projs.iter_projectees() {
                SolarSystem::util_remove_item_projection(u_data, svc, item_uid, projectee_uid);
                rev_projs.unreg_projectee(&item_uid, projectee_uid);
            }
        }
    }
    pub(in crate::api) fn util_add_item_projection(
        u_data: &UData,
        svc: &mut Svc,
        projector_uid: UItemId,
        projectee_uid: UItemId,
        proj_data: Option<UProjData>,
    ) {
        let projector_u_item = u_data.items.get(projector_uid);
        let projectee_u_item = u_data.items.get(projectee_uid);
        if let Some(reffs) = projector_u_item.get_reffs() {
            for &effect_rid in reffs.iter() {
                let r_effect = u_data.r_data.get_effect_by_rid(effect_rid);
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
                let r_effect = u_data.r_data.get_effect_by_rid(effect_rid);
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
        if let Some(reffs) = projector_u_item.get_reffs() {
            for &effect_rid in reffs.iter() {
                let r_effect = u_data.r_data.get_effect_by_rid(effect_rid);
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
