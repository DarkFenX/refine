use itertools::Itertools;

use crate::{
    ItemId, SkillLevel,
    ad::AItemId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UFit, UItemId},
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
#[derive(Clone)]
pub struct ValOverloadSkillFail {
    /// Current level of the Thermodynamics skill.
    pub td_lvl: Option<SkillLevel>,
    /// Overloaded modules which do not pass the check, and required Thermodynamics skill level.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub module_reqs: Vec<ValOverloadSkillItemInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Copy, Clone)]
pub struct ValOverloadSkillItemInfo {
    /// Overloaded item which fails the validation.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    /// Level of the Thermodynamics skill it needs to be overloadable.
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub level_req: SkillLevel,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_overload_skill_fast(&self, kfs: &RSet<UItemId>, fit: &UFit) -> bool {
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
    pub(in crate::svc::vast::val) fn validate_overload_skill_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        fit: &UFit,
    ) -> Option<ValOverloadSkillFail> {
        if self.overload_td_lvl.is_empty() {
            return None;
        }
        let td_lvl = fit.skills.get(&AItemId::THERMODYNAMICS).map(|v| v.level);
        let module_reqs = self
            .overload_td_lvl
            .iter()
            .filter_map(|(item_uid, req_lvl)| {
                match td_lvl.map(|td_lvl| td_lvl >= *req_lvl).unwrap_or(false) || kfs.contains(item_uid) {
                    true => None,
                    false => Some(ValOverloadSkillItemInfo {
                        item_id: ctx.u_data.items.ext_id_by_int_id(*item_uid),
                        level_req: *req_lvl,
                    }),
                }
            })
            .collect_vec();
        match module_reqs.is_empty() {
            true => None,
            false => Some(ValOverloadSkillFail { td_lvl, module_reqs }),
        }
    }
}
