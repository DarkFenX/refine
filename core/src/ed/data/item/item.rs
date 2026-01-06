use crate::{
    ed::{EFloat, EItemGrpId, EItemId},
    util::LibNamed,
};

pub struct EItem {
    pub id: EItemId,
    pub group_id: EItemGrpId,
    pub capacity: EFloat,
    pub mass: EFloat,
    pub radius: EFloat,
    pub volume: EFloat,
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
