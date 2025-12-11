use crate::{
    ad::{AItemId, AItemListId},
    util::RSet,
};

#[derive(Clone)]
pub struct AItemList {
    pub id: AItemListId,
    pub item_ids: RSet<AItemId>,
}
