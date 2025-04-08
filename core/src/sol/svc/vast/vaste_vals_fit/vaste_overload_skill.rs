use std::collections::HashMap;

use crate::{
    ac,
    sol::{
        ItemId, ItemKey, SkillLevel,
        svc::vast::VastFitData,
        uad::{Uad, fit::Fit},
    },
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
    pub(in crate::sol::svc::vast) fn validate_overload_skill_fast(&self, kfs: &RSet<ItemKey>, fit: &Fit) -> bool {
        if self.overload_td_lvl.is_empty() {
            return true;
        }
        let td_lvl = match fit.skills.get(&ac::items::THERMODYNAMICS) {
            Some(skill) => skill.level,
            None => return self.overload_td_lvl.is_subset(kfs),
        };
        self.overload_td_lvl
            .iter()
            .all(|(item_key, &req_lvl)| td_lvl >= req_lvl || kfs.contains(item_key))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_overload_skill_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        fit: &Fit,
    ) -> Option<ValOverloadSkillFail> {
        if self.overload_td_lvl.is_empty() {
            return None;
        }
        let td_lvl = fit.skills.get(&ac::items::THERMODYNAMICS).map(|v| v.level);
        let module_reqs: HashMap<_, _> = self
            .overload_td_lvl
            .iter()
            .filter(|(item_key, req_lvl)| match td_lvl {
                Some(td_lvl) => **req_lvl > td_lvl,
                None => true,
            } && !kfs.contains(item_key))
            .map(|(item_key, req_lvl)| (uad.items.id_by_key(*item_key), *req_lvl))
            .collect();
        match module_reqs.is_empty() {
            true => None,
            false => Some(ValOverloadSkillFail { td_lvl, module_reqs }),
        }
    }
}
