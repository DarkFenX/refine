use crate::{
    err::basic::ItemFoundError,
    sol::uad::{container::EntityContainer, item::UadItem},
};

pub(in crate::sol) type Items = EntityContainer<UadItem, ItemFoundError>;
