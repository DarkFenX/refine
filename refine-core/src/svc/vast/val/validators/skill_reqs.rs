use itertools::Itertools;

use crate::{
    ItemId, SkillLevel,
    api::ItemTypeId,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::RSet,
};

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct ValSrqSkillStored {
    pub(in crate::svc::vast) current_lvl: Option<SkillLevel>,
    pub(in crate::svc::vast) required_lvl: SkillLevel,
}

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValSrqFail {
    /// Items and their unsatisfied skill requirements, which are defined as another list of skills
    /// and info about levels.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub items: Vec<ValSrqItemInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Clone)]
pub struct ValSrqItemInfo {
    /// Item with unsatisfied skill requirements
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    /// List of missing skills.
    #[cfg_attr(feature = "serde", vec_map(value, serialize_as = "serde_with::KeyValueMap<_>"))]
    pub missing_skills: Vec<ValSrqSkillInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValSrqSkillInfo {
    pub skill_type_id: ItemTypeId,
    /// Current skill level, None if skill is absent on the fit.
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
            .filter_map(|(item_uid, missing_skills)| match kfs.contains(item_uid) {
                true => None,
                false => Some(ValSrqItemInfo {
                    item_id: ctx.u_data.items.ext_id_by_int_id(*item_uid),
                    missing_skills: missing_skills
                        .iter()
                        .map(|(skill_item_aid, skill_info)| ValSrqSkillInfo {
                            skill_type_id: ItemTypeId::from_aid(*skill_item_aid),
                            current_lvl: skill_info.current_lvl,
                            required_lvl: skill_info.required_lvl,
                        })
                        .collect_vec(),
                }),
            })
            .collect_vec();
        match items.is_empty() {
            true => None,
            false => Some(ValSrqFail { items }),
        }
    }
}
