use itertools::Itertools;

use crate::{
    ac,
    sol::{ItemId, SkillLevel, svc::vast::VastFitData, uad::fit::Fit},
    util::HSet,
};

pub struct ValOverloadSkillFail {
    pub td_lvl: Option<SkillLevel>,
    pub items: Vec<ValOverloadSkillItemInfo>,
}

pub struct ValOverloadSkillItemInfo {
    pub item_id: ItemId,
    pub req_lvl: SkillLevel,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_overload_skill_fast(&self, kfs: &HSet<ItemId>, fit: &Fit) -> bool {
        if self.overload_td_lvl.is_empty() {
            return true;
        }
        let td_lvl = match fit.skills.get(&ac::items::THERMODYNAMICS) {
            Some(skill) => skill.level,
            None => return self.overload_td_lvl.is_subset(kfs),
        };
        self.overload_td_lvl
            .iter()
            .all(|(item_id, &req_lvl)| td_lvl >= req_lvl || kfs.contains(item_id))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_overload_skill_verbose(
        &self,
        kfs: &HSet<ItemId>,
        fit: &Fit,
    ) -> Option<ValOverloadSkillFail> {
        if self.overload_td_lvl.is_empty() {
            return None;
        }
        let td_lvl = fit.skills.get(&ac::items::THERMODYNAMICS).map(|v| v.level);
        let items = self
            .overload_td_lvl
            .iter()
            .filter(|(item_id, req_lvl)| match td_lvl {
                Some(td_lvl) => **req_lvl > td_lvl,
                None => true,
            } && !kfs.contains(item_id))
            .map(|(&item_id, &req_lvl)| ValOverloadSkillItemInfo { item_id, req_lvl })
            .collect_vec();
        if items.is_empty() {
            return None;
        }
        Some(ValOverloadSkillFail { td_lvl, items })
    }
}
