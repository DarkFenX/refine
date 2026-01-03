use crate::{
    ed::{EGenFloat, EItemGrpId, EItemId},
    util::Named,
};

pub struct EItem {
    pub id: EItemId,
    pub group_id: EItemGrpId,
    pub capacity: EGenFloat,
    pub mass: EGenFloat,
    pub radius: EGenFloat,
    pub volume: EGenFloat,
}
impl Named for EItem {
    fn get_name() -> &'static str {
        "EItem"
    }
}
impl std::fmt::Display for EItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}
