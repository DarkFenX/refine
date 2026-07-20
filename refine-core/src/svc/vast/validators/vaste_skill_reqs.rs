use itertools::Itertools;

use crate::{
    api::ItemTypeId,
    num::SkillLevel,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
pub struct ValSrqFail {
    /// Items and their unsatisfied skill requirements, which are defined as another list of skills
    /// and info about levels.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, serde_with::Map<_, _>>"))]
    pub items: Vec<(ItemId, Vec<(ItemTypeId, ValSrqSkillInfo)>)>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
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
        let items = self
            .srqs_missing
            .iter()
            .filter(|(item_uid, _)| !kfs.contains(item_uid))
            .map(|(item_uid, missing_skills)| {
                (
                    ctx.u_data.items.ext_id_by_int_id(*item_uid),
                    missing_skills
                        .iter()
                        .map(|(skill_item_aid, skill_info)| (ItemTypeId::from_aid(*skill_item_aid), *skill_info))
                        .collect_vec(),
                )
            })
            .collect_vec();
        match items.is_empty() {
            true => None,
            false => Some(ValSrqFail { items }),
        }
    }
}
