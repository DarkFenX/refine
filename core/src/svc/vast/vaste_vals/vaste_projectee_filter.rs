use crate::{
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemKey,
    util::RSet,
};

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_projectee_filter_fast(&self, kfs: &RSet<UItemKey>, ctx: SvcCtx) -> bool {
        for (projector_espec, projectee_data) in self.projectee_filter.iter() {
            for (&projectee_key, allowed_type_list_id) in projectee_data.iter() {
                // Can't fetch type list - assume it's empty, i.e. effect has no allowed targets
                let allowed_type_list = match ctx.u_data.src.get_item_list(allowed_type_list_id) {
                    Some(allowed_type_list) => allowed_type_list,
                    None => return false,
                };
                let projectee_type_id = ctx.u_data.items.get(projectee_key).get_type_id();
                if !allowed_type_list.get_item_ids().contains(&projectee_type_id)
                    && !kfs.contains(&projector_espec.item_key)
                {
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
    ) -> Option<bool> {
        None
    }
}
