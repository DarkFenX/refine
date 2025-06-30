use crate::{
    err::basic::ItemFoundError,
    uad::{container::EntityContainer, item::UadItem},
};

pub(crate) type Items = EntityContainer<UadItem, ItemFoundError>;
