use crate::{
    ad::{AItemId, AItemListId},
    util::RSet,
};

pub(in crate::adg) struct GItemList {
    pub(in crate::adg) id: AItemListId,
    pub(in crate::adg) item_ids: RSet<AItemId>,
}
