use crate::{
    err::basic::FleetFoundError,
    sol::uad::{container::EntityContainer, fleet::UadFleet},
};

pub(in crate::sol) type Fleets = EntityContainer<UadFleet, FleetFoundError>;
