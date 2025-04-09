use crate::{
    err::basic::ItemFoundError,
    sol::uad::{container::EntityContainer, item::Item},
};

pub(in crate::sol) type Items = EntityContainer<Item, ItemFoundError>;
