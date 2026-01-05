use crate::{
    err::basic::FleetFoundError,
    ud::{FleetId, UFleet, UFleetId, container::UEntityContainer},
};

pub(crate) type UFleets = UEntityContainer<UFleet, FleetId, UFleetId, FleetFoundError>;
