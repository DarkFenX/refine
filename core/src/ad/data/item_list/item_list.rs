use crate::{
    ad::{AItemId, AItemListId},
    util::RSet,
};

pub struct AItemList {
    pub id: AItemListId,
    pub item_ids: RSet<AItemId>,
}
