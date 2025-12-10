use crate::{
    ed::{EAttrVal, EItemGrpId, EItemId},
    util::Named,
};

pub struct EItem {
    pub id: EItemId,
    pub group_id: EItemGrpId,
    pub capacity: EAttrVal,
    pub mass: EAttrVal,
    pub radius: EAttrVal,
    pub volume: EAttrVal,
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
