use crate::{
    ed::{EFloat, EItemGrpId, EItemId},
    util::LibNamed,
};

pub struct EItem {
    pub id: EItemId,
    pub group_id: EItemGrpId,
    pub capacity: Option<EFloat>,
    pub mass: Option<EFloat>,
    pub radius: Option<EFloat>,
    pub volume: Option<EFloat>,
}
impl LibNamed for EItem {
    fn lib_get_name() -> &'static str {
        "EItem"
    }
}
impl std::fmt::Display for EItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::lib_get_name(), self.id)
    }
}
