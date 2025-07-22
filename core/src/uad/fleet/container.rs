use crate::{
    err::basic::FleetFoundError,
    uad::{container::UadEntityContainer, fleet::UadFleet},
};

pub(crate) type UadFleets = UadEntityContainer<UadFleet, FleetFoundError>;
