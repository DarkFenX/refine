use itertools::Itertools;

use crate::{
    ItemId,
    rd::RItemListId,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::{RMap, RSet},
    val::{ValItemStatus, ValResult},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Result
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValProjFilterFail {
    /// Projecting items and targets they can't be projected to.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub items: Vec<ValProjFilterItemInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Clone)]
pub struct ValProjFilterItemInfo {
    /// Item-projector which fails the validation.
    pub item_id: ItemId,
    pub status: ValItemStatus,
    /// Projectee item IDs the projector can't be projected to.
    pub projectee_item_ids: Vec<ItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_projectee_filter_fast(&self, kfs: &RSet<UItemId>, ctx: &SvcCtx) -> bool {
        for (&projector_espec, projectee_data) in self.projectee_filter.iter() {
            for (&projectee_uid, &allowed_type_list_rid) in projectee_data.iter() {
                if !is_projectee_allowed(ctx, allowed_type_list_rid, projectee_uid)
                    && !kfs.contains(&projector_espec.item_uid)
                {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_projectee_filter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
    ) -> ValResult<ValProjFilterFail> {
        let mut items = RMap::new();
        for (&projector_espec, projectee_data) in self.projectee_filter.iter() {
            for (&projectee_uid, &allowed_type_list_rid) in projectee_data.iter() {
                if !is_projectee_allowed(ctx, allowed_type_list_rid, projectee_uid) {
                    let projectee_item_id = ctx.u_data.items.ext_id_by_int_id(projectee_uid);
                    items
                        .entry(projector_espec.item_uid)
                        .or_insert_with(RSet::new)
                        .insert(projectee_item_id);
                }
            }
        }
        let items = items
            .into_iter()
            .map(|(projector_uid, projectee_item_ids)| ValProjFilterItemInfo {
                item_id: ctx.u_data.items.ext_id_by_int_id(projector_uid),
                status: ValItemStatus::from_kfs(kfs, projector_uid),
                projectee_item_ids: projectee_item_ids.into_iter().collect(),
            })
            .collect_vec();
        match items.is_empty() {
            true => ValResult::Passed,
            false => ValResult::from_fail(items.iter().all(|v| v.status.is_kf()), ValProjFilterFail { items }),
        }
    }
}

fn is_projectee_allowed(ctx: &SvcCtx, allowed_type_list_rid: RItemListId, projectee_uid: UItemId) -> bool {
    let allowed_type_list = ctx.u_data.r_data.get_item_list_by_rid(allowed_type_list_rid);
    let projectee_type_aid = ctx.u_data.items.get(projectee_uid).get_rendered_type_aid();
    allowed_type_list.item_aids.contains(&projectee_type_aid)
}
