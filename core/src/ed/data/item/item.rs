use crate::{
    ed::{EGenFloat, EItemGrpId, EItemId},
    util::LibNamed,
};

pub struct EItem {
    pub id: EItemId,
    pub group_id: EItemGrpId,
    pub capacity: EGenFloat,
    pub mass: EGenFloat,
    pub radius: EGenFloat,
    pub volume: EGenFloat,
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
