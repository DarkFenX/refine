use crate::{
    err::basic::FleetFoundError,
    sol::uad::{container::EntityContainer, fleet::Fleet},
};

pub(in crate::sol) type Fleets = EntityContainer<Fleet, FleetFoundError>;
