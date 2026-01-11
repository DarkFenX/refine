use std::collections::HashMap;

use crate::{
    ad::AItemId,
    num::SkillLevel,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UFit, UItemId},
    util::RSet,
};

pub struct ValOverloadSkillFail {
    /// Current level of the Thermodynamics skill.
    pub td_lvl: Option<SkillLevel>,
    /// Map between item IDs of overloaded modules which do not pass the check, and required
    /// Thermodynamics skill level.
    pub module_reqs: HashMap<ItemId, SkillLevel>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_overload_skill_fast(&self, kfs: &RSet<UItemId>, fit: &UFit) -> bool {
        if self.overload_td_lvl.is_empty() {
            return true;
        }
        let td_lvl = match fit.skills.get(&AItemId::THERMODYNAMICS) {
            Some(skill) => skill.level,
            None => return self.overload_td_lvl.is_subset(kfs),
        };
        self.overload_td_lvl
            .iter()
            .all(|(item_uid, &req_lvl)| td_lvl >= req_lvl || kfs.contains(item_uid))
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_overload_skill_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        fit: &UFit,
    ) -> Option<ValOverloadSkillFail> {
        if self.overload_td_lvl.is_empty() {
            return None;
        }
        let td_lvl = fit.skills.get(&AItemId::THERMODYNAMICS).map(|v| v.level);
        let module_reqs: HashMap<_, _> = self
            .overload_td_lvl
            .iter()
            .filter(|(item_uid, req_lvl)| match td_lvl {
                Some(td_lvl) => **req_lvl > td_lvl,
                None => true,
            } && !kfs.contains(item_uid))
            .map(|(&item_uid, &req_lvl)| (ctx.u_data.items.xid_by_iid(item_uid), req_lvl.into()))
            .collect();
        match module_reqs.is_empty() {
            true => None,
            false => Some(ValOverloadSkillFail { td_lvl, module_reqs }),
        }
    }
}
