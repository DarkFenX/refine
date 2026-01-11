use std::collections::HashMap;

use crate::{
    api::ItemTypeId,
    num::SkillLevel,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

pub struct ValSrqFail {
    /// Map between item IDs and their unsatisfied skill requirements, which are defined as another
    /// map, with keys being skill type IDs, and values containing further info about levels..
    pub items: HashMap<ItemId, HashMap<ItemTypeId, ValSrqSkillInfo>>,
}
#[derive(Copy, Clone)]
pub struct ValSrqSkillInfo {
    /// Current skill level, None if skill is absent on fit.
    pub current_lvl: Option<SkillLevel>,
    /// Skill level required by the item.
    pub required_lvl: SkillLevel,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_skill_reqs_fast(&self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.srqs_missing.is_empty(),
            false => self.srqs_missing.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_skill_reqs_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValSrqFail> {
        let items: HashMap<_, _> = self
            .srqs_missing
            .iter()
            .filter(|(item_uid, _)| !kfs.contains(item_uid))
            .map(|(item_uid, missing_skills)| {
                (
                    ctx.u_data.items.xid_by_iid(*item_uid),
                    missing_skills
                        .iter()
                        .map(|(skill_item_aid, skill_info)| (ItemTypeId::from_aid(*skill_item_aid), *skill_info))
                        .collect(),
                )
            })
            .collect();
        match items.is_empty() {
            true => None,
            false => Some(ValSrqFail { items }),
        }
    }
}
