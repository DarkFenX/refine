use crate::{
    def::FleetId,
    err::basic::FleetFoundError,
    ud::{UFleet, UFleetId, container::UEntityContainer},
};

pub(crate) type UFleets = UEntityContainer<UFleet, FleetId, UFleetId, FleetFoundError>;
