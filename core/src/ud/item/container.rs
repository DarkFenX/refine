use crate::{
    err::basic::ItemFoundError,
    ud::{container::UEntityContainer, item::UItem},
};

pub(crate) type UItems = UEntityContainer<UItem, ItemFoundError>;
