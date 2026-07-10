use crate::{
    ed::{EItemCatId, EItemGrpId},
    util::LibNamed,
};

pub struct EItemGroup {
    pub id: EItemGrpId,
    pub category_id: EItemCatId,
}
impl LibNamed for EItemGroup {
    fn lib_get_name() -> &'static str {
        "EItemGroup"
    }
}
