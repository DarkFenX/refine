use crate::{
    def::FleetId,
    err::basic::FleetFoundError,
    ud::{UFleet, UFleetKey, container::UEntityContainer},
};

pub(crate) type UFleets = UEntityContainer<UFleet, UFleetKey, FleetId, FleetFoundError>;
