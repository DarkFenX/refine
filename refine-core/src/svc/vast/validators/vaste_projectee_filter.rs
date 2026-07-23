use crate::{
    ItemId,
    misc::EffectSpec,
    rd::RItemListId,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::{RMap, RSet},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct ValProjFilterFail {
    /// Projecting items and targets they can't be projected to.
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::as_map"))]
    pub items: Vec<ValProjFilterItemInfo>,
}

pub struct ValProjFilterItemInfo {
    /// Item-projector which fails the validation.
    pub item_id: ItemId,
    /// Projectee item IDs the projector can't be projected to.
    pub projectee_item_ids: Vec<ItemId>,
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
        let mut items = RMap::new();
        for (projector_espec, projectee_data) in self.projectee_filter.iter() {
            for (&projectee_uid, &allowed_type_list_id) in projectee_data.iter() {
                if !validate_projection(kfs, ctx, projector_espec, allowed_type_list_id, projectee_uid) {
                    let projector_item_id = ctx.u_data.items.ext_id_by_int_id(projector_espec.item_uid);
                    let projectee_item_ids = items.entry(projector_item_id).or_insert_with(Vec::new);
                    let projectee_item_id = ctx.u_data.items.ext_id_by_int_id(projectee_uid);
                    if !projectee_item_ids.contains(&projectee_item_id) {
                        projectee_item_ids.push(projectee_item_id)
                    }
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
                        projectee_item_ids,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{SerializeMap, Serializer};

    use super::*;

    pub(super) fn as_map<S>(items: &[ValProjFilterItemInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(items.len()))?;
        for item in items {
            map.serialize_entry(&item.item_id, &item.projectee_item_ids)?;
        }
        map.end()
    }
}
