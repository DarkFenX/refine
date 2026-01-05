use crate::{
    api::ItemId,
    err::basic::ItemFoundError,
    ud::{UItem, UItemId, container::UEntityContainer},
};

pub(crate) type UItems = UEntityContainer<UItem, ItemId, UItemId, ItemFoundError>;
