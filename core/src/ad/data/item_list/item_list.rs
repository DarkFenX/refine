use crate::{
    ad::{AItemId, AItemListId},
    util::{Named, RSet},
};

pub struct AItemList {
    pub id: AItemListId,
    pub item_ids: RSet<AItemId>,
}
impl Named for AItemList {
    fn get_name() -> &'static str {
        "AItemList"
    }
}
impl std::fmt::Display for AItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={})", Self::get_name(), self.id)
    }
}
