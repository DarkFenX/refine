use crate::{
    err::basic::FleetFoundError,
    uad::{container::EntityContainer, fleet::UadFleet},
};

pub(crate) type Fleets = EntityContainer<UadFleet, FleetFoundError>;
