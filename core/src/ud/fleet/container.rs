use crate::{
    err::basic::FleetFoundError,
    ud::{container::UEntityContainer, fleet::UFleet},
};

pub(crate) type UFleets = UEntityContainer<UFleet, FleetFoundError>;
