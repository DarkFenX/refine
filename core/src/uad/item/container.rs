use crate::{
    err::basic::ItemFoundError,
    uad::{container::EntityContainer, item::UadItem},
};

pub(crate) type UadItems = EntityContainer<UadItem, ItemFoundError>;
