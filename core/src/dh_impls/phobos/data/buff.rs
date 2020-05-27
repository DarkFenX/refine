use crate::defines::ReeInt;
use crate::dh;

use super::super::fsd::FsdMerge;
use super::aux::into_vec;

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct Buff {
    #[serde(rename = "aggregateMode")]
    pub(in super::super) aggregate_mode: String,
    #[serde(rename = "operationName")]
    pub(in super::super) operation: String,
    #[serde(rename = "itemModifiers")]
    pub(in super::super) item_mods: Vec<BuffIM>,
    #[serde(rename = "locationModifiers")]
    pub(in super::super) loc_mods: Vec<BuffLM>,
    #[serde(rename = "locationGroupModifiers")]
    pub(in super::super) locgroup_mods: Vec<BuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers")]
    pub(in super::super) locsrq_mods: Vec<BuffLRSM>,
}
impl FsdMerge<dh::Buff> for Buff {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::Buff> {
        vec![dh::Buff::new(
            id,
            self.aggregate_mode,
            self.operation,
            into_vec(self.item_mods),
            into_vec(self.loc_mods),
            into_vec(self.locgroup_mods),
            into_vec(self.locsrq_mods),
        )]
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct BuffIM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in super::super) attr_id: ReeInt,
}
impl Into<dh::BuffIM> for BuffIM {
    fn into(self) -> dh::BuffIM {
        dh::BuffIM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct BuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in super::super) attr_id: ReeInt,
}
impl Into<dh::BuffLM> for BuffLM {
    fn into(self) -> dh::BuffLM {
        dh::BuffLM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct BuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in super::super) attr_id: ReeInt,
    #[serde(rename = "groupID")]
    pub(in super::super) group_id: ReeInt,
}
impl Into<dh::BuffLGM> for BuffLGM {
    fn into(self) -> dh::BuffLGM {
        dh::BuffLGM::new(self.attr_id, self.group_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct BuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(in super::super) attr_id: ReeInt,
    #[serde(rename = "skillID")]
    pub(in super::super) skill_id: ReeInt,
}
impl Into<dh::BuffLRSM> for BuffLRSM {
    fn into(self) -> dh::BuffLRSM {
        dh::BuffLRSM::new(self.attr_id, self.skill_id)
    }
}
