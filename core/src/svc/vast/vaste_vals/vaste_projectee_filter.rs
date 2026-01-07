use std::collections::HashMap;

use crate::{
    misc::EffectSpec,
    rd::RItemListId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

pub struct ValProjFilterFail {
    /// Map between projecting item IDs and targets they can't be projected to.
    pub items: HashMap<ItemId, Vec<ItemId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_projectee_filter_fast(&self, kfs: &RSet<UItemId>, ctx: SvcCtx) -> bool {
        for (projector_espec, projectee_data) in self.projectee_filter.iter() {
            for (&projectee_uid, &allowed_type_list_rid) in projectee_data.iter() {
                if !validate_projection(kfs, ctx, projector_espec, allowed_type_list_rid, projectee_uid) {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_projectee_filter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValProjFilterFail> {
        let mut items = HashMap::new();
        for (projector_espec, projectee_data) in self.projectee_filter.iter() {
            for (&projectee_uid, &allowed_type_list_id) in projectee_data.iter() {
                if !validate_projection(kfs, ctx, projector_espec, allowed_type_list_id, projectee_uid) {
                    let projector_item_id = ctx.u_data.items.xid_by_iid(projector_espec.item_uid);
                    let projectee_item_ids = items.entry(projector_item_id).or_insert_with(Vec::new);
                    let projectee_item_id = ctx.u_data.items.xid_by_iid(projectee_uid);
                    if !projectee_item_ids.contains(&projectee_item_id) {
                        projectee_item_ids.push(projectee_item_id)
                    }
                }
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValProjFilterFail { items }),
        }
    }
}

fn validate_projection(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    projector_espec: &EffectSpec,
    allowed_type_list_rid: RItemListId,
    projectee_uid: UItemId,
) -> bool {
    let allowed_type_list = ctx.u_data.src.get_item_list_by_rid(allowed_type_list_rid);
    let projectee_type_id = ctx.u_data.items.get(projectee_uid).get_type_id();
    allowed_type_list.item_aids.contains(&projectee_type_id) || kfs.contains(&projector_espec.item_uid)
}
