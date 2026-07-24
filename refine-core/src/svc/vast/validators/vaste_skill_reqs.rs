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

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct ValSrqFail {
    /// Items and their unsatisfied skill requirements, which are defined as another list of skills
    /// and info about levels.
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::as_nested_map"))]
    pub items: Vec<ValSrqItemInfo>,
}

pub struct ValSrqItemInfo {
    /// Item with unsatisfied skill requirements
    pub item_id: ItemId,
    /// List of missing skills..
    pub missing_skills: Vec<ValSrqSkillInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Serialize, SerializeMap, Serializer};

    use super::*;

    pub(super) fn as_nested_map<S>(items: &[ValSrqItemInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(items.len()))?;
        for item in items {
            map.serialize_entry(&item.item_id, &SkillInfo(&item.missing_skills))?;
        }
        map.end()
    }

    struct SkillInfo<'a>(&'a [ValSrqSkillInfo]);
    impl Serialize for SkillInfo<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.0.len()))?;
            for skill in self.0 {
                map.serialize_entry(&skill.skill_type_id, &(skill.current_lvl, skill.required_lvl))?;
            }
            map.end()
        }
    }
}
