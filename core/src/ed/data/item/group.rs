use crate::{
    ed::{EItemCatId, EItemGrpId},
    util::Named,
};

pub struct EItemGroup {
    pub id: EItemGrpId,
    pub category_id: EItemCatId,
}
impl Named for EItemGroup {
    fn get_name() -> &'static str {
        "EItemGroup"
    }
}
