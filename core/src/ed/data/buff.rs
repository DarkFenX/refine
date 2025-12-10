use crate::{
    ed::{EAttrId, EBuffId, EItemGrpId, EItemId},
    util::Named,
};

pub struct EBuff {
    pub id: EBuffId,
    pub aggregate_mode: String,
    pub operation: String,
    pub item_mods: Vec<EBuffIM>,
    pub loc_mods: Vec<EBuffLM>,
    pub locgroup_mods: Vec<EBuffLGM>,
    pub locsrq_mods: Vec<EBuffLRSM>,
}
impl Named for EBuff {
    fn get_name() -> &'static str {
        "EBuff"
    }
}
impl std::fmt::Display for EBuff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}

pub struct EBuffIM {
    pub attr_id: EAttrId,
}

pub struct EBuffLM {
    pub attr_id: EAttrId,
}

pub struct EBuffLGM {
    pub attr_id: EAttrId,
    pub group_id: EItemGrpId,
}

pub struct EBuffLRSM {
    pub attr_id: EAttrId,
    pub skill_id: EItemId,
}
