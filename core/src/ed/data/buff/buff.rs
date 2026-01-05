use crate::{
    ed::{EBuffIM, EBuffId, EBuffLGM, EBuffLM, EBuffLRSM},
    util::LibNamed,
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
impl LibNamed for EBuff {
    fn lib_get_name() -> &'static str {
        "EBuff"
    }
}
impl std::fmt::Display for EBuff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::lib_get_name(), self.id)
    }
}
