use crate::{
    ItemId,
    misc::EffectSpec,
    rd::RItemListId,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::{RMap, RSet},
};

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
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub items: Vec<ValProjFilterItemInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Clone)]
pub struct ValProjFilterItemInfo {
    /// Item-projector which fails the validation.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    /// Projectee item IDs the projector can't be projected to.
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub projectee_item_ids: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_projectee_filter_fast(&self, kfs: &RSet<UItemId>, ctx: SvcCtx) -> bool {
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
    pub(in crate::svc::vast::val) fn validate_projectee_filter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValProjFilterFail> {
        let mut items = RMap::new();
        for (projector_espec, projectee_data) in self.projectee_filter.iter() {
            for (&projectee_uid, &allowed_type_list_id) in projectee_data.iter() {
                if !validate_projection(kfs, ctx, projector_espec, allowed_type_list_id, projectee_uid) {
                    let projector_item_id = ctx.u_data.items.ext_id_by_int_id(projector_espec.item_uid);
                    let projectee_item_id = ctx.u_data.items.ext_id_by_int_id(projectee_uid);
                    items
                        .entry(projector_item_id)
                        .or_insert_with(RSet::new)
                        .insert(projectee_item_id);
                }
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValProjFilterFail {
                items: items
                    .into_iter()
                    .map(|(projector_item_id, projectee_item_ids)| ValProjFilterItemInfo {
                        item_id: projector_item_id,
                        projectee_item_ids: projectee_item_ids.into_iter().collect(),
                    })
                    .collect(),
            }),
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
    let allowed_type_list = ctx.u_data.r_data.get_item_list_by_rid(allowed_type_list_rid);
    let projectee_type_aid = ctx.u_data.items.get(projectee_uid).get_type_aid();
    allowed_type_list.item_aids.contains(&projectee_type_aid) || kfs.contains(&projector_espec.item_uid)
}
