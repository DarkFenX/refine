use crate::{
    ed::{EBuffId, EBuffIm, EBuffLgm, EBuffLm, EBuffLrsm},
    util::LibNamed,
};

pub struct EBuff {
    pub id: EBuffId,
    pub aggregate_mode: String,
    pub operation: String,
    pub item_mods: Vec<EBuffIm>,
    pub loc_mods: Vec<EBuffLm>,
    pub locgroup_mods: Vec<EBuffLgm>,
    pub locsrq_mods: Vec<EBuffLrsm>,
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
