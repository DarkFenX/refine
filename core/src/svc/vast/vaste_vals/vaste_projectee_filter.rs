use std::collections::HashMap;

use crate::{
    def::ItemId,
    misc::EffectSpec,
    rd::RItemListKey,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemKey,
    util::RSet,
};

pub struct ValProjFilterFail {
    /// Map between projecting item IDs and targets they can't be projected to.
    pub items: HashMap<ItemId, Vec<ItemId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_projectee_filter_fast(&self, kfs: &RSet<UItemKey>, ctx: SvcCtx) -> bool {
        for (projector_espec, projectee_data) in self.projectee_filter.iter() {
            for (&projectee_key, &allowed_type_list_key) in projectee_data.iter() {
                if !validate_projection(kfs, ctx, projector_espec, allowed_type_list_key, projectee_key) {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_projectee_filter_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValProjFilterFail> {
        let mut items = HashMap::new();
        for (projector_espec, projectee_data) in self.projectee_filter.iter() {
            for (&projectee_key, &allowed_type_list_id) in projectee_data.iter() {
                if !validate_projection(kfs, ctx, projector_espec, allowed_type_list_id, projectee_key) {
                    let projector_item_id = ctx.u_data.items.id_by_key(projector_espec.item_key);
                    let projectee_item_ids = items.entry(projector_item_id).or_insert_with(Vec::new);
                    let projectee_item_id = ctx.u_data.items.id_by_key(projectee_key);
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
    kfs: &RSet<UItemKey>,
    ctx: SvcCtx,
    projector_espec: &EffectSpec,
    allowed_type_list_key: RItemListKey,
    projectee_key: UItemKey,
) -> bool {
    let allowed_type_list = ctx.u_data.src.get_item_list(allowed_type_list_key);
    let projectee_type_id = ctx.u_data.items.get(projectee_key).get_type_id();
    allowed_type_list.item_ids.contains(&projectee_type_id) || kfs.contains(&projector_espec.item_key)
}
