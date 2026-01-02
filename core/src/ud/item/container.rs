use crate::{
    def::ItemId,
    err::basic::ItemFoundError,
    ud::{UItem, UItemKey, container::UEntityContainer},
};

pub(crate) type UItems = UEntityContainer<UItem, UItemKey, ItemId, ItemFoundError>;
