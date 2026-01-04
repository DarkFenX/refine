#[derive(thiserror::Error, Debug)]
pub(crate) enum HExecError {
    // Fits
    #[error("{0}")]
    FitNotFoundPrimary(#[source] rc::err::basic::FitFoundError),
    #[error("{0}")]
    FitNotFoundSecondary(#[source] rc::err::basic::FitFoundError),
    #[error("fit {0} has no character set")]
    FitCharacterNotFound(rc::FitId),
    #[error("fit {0} has no ship set")]
    FitShipNotFound(rc::FitId),
    #[error("fit {0} has no stance set")]
    FitStanceNotFound(rc::FitId),
    #[error("{0}")]
    FitNotInFleet(#[source] rc::err::basic::FitFleetAssignedError),
    #[error("{0}")]
    FitNotInThisFleet(#[source] rc::err::basic::FitInThisFleetError),
    #[error("{0}")]
    FitAlreadyInThisFleet(#[source] rc::err::basic::FitNotInThisFleetError),
    // Fleets
    #[error("{0}")]
    FleetNotFoundPrimary(#[source] rc::err::basic::FleetFoundError),
    #[error("{0}")]
    FleetNotFoundSecondary(#[source] rc::err::basic::FleetFoundError),
    // Items
    #[error("{0}")]
    ItemNotFoundPrimary(#[source] rc::err::basic::ItemFoundError),
    #[error("{0}")]
    ItemNotFoundSecondary(#[source] rc::err::basic::ItemFoundError),
    #[error("{0}")]
    ItemKindMismatch(#[source] rc::err::basic::ItemKindMatchError),
    #[error("{0}")]
    SkillIdCollision(#[source] rc::err::basic::SkillEveTypeError),
    #[error("item {0} is not mutated")]
    MutationNotSet(rc::ItemId),
    #[error("item {0} does not have charge set")]
    ChargeNotSet(rc::ItemId),
    #[error("autocharge cannot be manually removed")]
    UnremovableAutocharge,
    #[error("{0}")]
    InvalidFighterCount(#[from] rc::err::FighterCountError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[source] rc::err::basic::ItemReceiveProjError),
    #[error("{0}")]
    ProjectionNotFound(#[source] rc::err::basic::ProjFoundError),
    #[error("{0}")]
    ProjectionAlreadyExists(#[source] rc::err::basic::ProjNotFoundError),
}
impl HExecError {
    pub(crate) fn get_code(&self) -> String {
        match self {
            // Fits
            HExecError::FitNotFoundPrimary(_) => "EXC-002",
            HExecError::FitNotFoundSecondary(_) => "EXC-003",
            HExecError::FitCharacterNotFound(_) => "EXC-004",
            HExecError::FitShipNotFound(_) => "EXC-005",
            HExecError::FitStanceNotFound(_) => "EXC-006",
            HExecError::FitNotInFleet(_) => "EXC-007",
            HExecError::FitNotInThisFleet(_) => "EXC-008",
            HExecError::FitAlreadyInThisFleet(_) => "EXC-008.1",
            // Fleets
            HExecError::FleetNotFoundPrimary(_) => "EXC-010",
            HExecError::FleetNotFoundSecondary(_) => "EXC-011",
            // Items
            HExecError::ItemNotFoundPrimary(_) => "EXC-013",
            HExecError::ItemNotFoundSecondary(_) => "EXC-014",
            HExecError::ItemKindMismatch(_) => "EXC-015",
            HExecError::SkillIdCollision(_) => "EXC-015.1",
            HExecError::MutationNotSet(_) => "MUT-001",
            HExecError::ChargeNotSet(_) => "NCH-001",
            HExecError::UnremovableAutocharge => "ACH-001",
            HExecError::InvalidFighterCount(_) => "FTR-019",
            HExecError::ProjecteeCantTakeProjs(_) => "EXC-021",
            HExecError::ProjectionNotFound(_) => "EXC-022",
            HExecError::ProjectionAlreadyExists(_) => "EXC-023",
        }
        .to_string()
    }
}
