use crate::{
    err::basic::ItemFoundError,
    uad::{container::UadEntityContainer, item::UadItem},
};

pub(crate) type UadItems = UadEntityContainer<UadItem, ItemFoundError>;
