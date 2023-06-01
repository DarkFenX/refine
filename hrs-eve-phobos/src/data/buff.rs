use crate::{fsd::FsdMerge, util::into_vec};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Buff {
    #[serde(rename = "aggregateMode")]
    pub(crate) aggregate_mode: String,
    #[serde(rename = "operationName")]
    pub(crate) operation: String,
    #[serde(rename = "itemModifiers")]
    pub(crate) item_mods: Vec<BuffIM>,
    #[serde(rename = "locationModifiers")]
    pub(crate) loc_mods: Vec<BuffLM>,
    #[serde(rename = "locationGroupModifiers")]
    pub(crate) locgroup_mods: Vec<BuffLGM>,
    #[serde(rename = "locationRequiredSkillModifiers")]
    pub(crate) locsrq_mods: Vec<BuffLRSM>,
}
impl FsdMerge<rc::edt::Buff> for Buff {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::Buff> {
        vec![rc::edt::Buff::new(
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
pub(crate) struct BuffIM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
}
impl Into<rc::edt::BuffIM> for BuffIM {
    fn into(self) -> rc::edt::BuffIM {
        rc::edt::BuffIM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BuffLM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
}
impl Into<rc::edt::BuffLM> for BuffLM {
    fn into(self) -> rc::edt::BuffLM {
        rc::edt::BuffLM::new(self.attr_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BuffLGM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
    #[serde(rename = "groupID")]
    pub(crate) group_id: rc::ReeInt,
}
impl Into<rc::edt::BuffLGM> for BuffLGM {
    fn into(self) -> rc::edt::BuffLGM {
        rc::edt::BuffLGM::new(self.attr_id, self.group_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BuffLRSM {
    #[serde(rename = "dogmaAttributeID")]
    pub(crate) attr_id: rc::ReeInt,
    #[serde(rename = "skillID")]
    pub(crate) skill_id: rc::ReeInt,
}
impl Into<rc::edt::BuffLRSM> for BuffLRSM {
    fn into(self) -> rc::edt::BuffLRSM {
        rc::edt::BuffLRSM::new(self.attr_id, self.skill_id)
    }
}
