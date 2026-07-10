use crate::{
    err::basic::ItemFoundError,
    ud::{ItemId, UItem, UItemId, container::UEntityContainer},
};

pub(crate) type UItems = UEntityContainer<UItem, ItemId, UItemId, ItemFoundError>;
