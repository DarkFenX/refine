#[derive(Debug, thiserror::Error)]
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
    // Backreferences
    #[error("referenced command #{0} does not have results recorded")]
    BackrefCmdNotFound(usize),
    #[error("referenced command #{0} exists, but does not have fit ID info")]
    BackrefCmdNoFitId(usize),
    #[error("referenced command #{0} exists, but does not have fleet ID info")]
    BackrefCmdNoFleetId(usize),
    #[error("referenced command #{0} exists, but does not have item ID info")]
    BackrefCmdNoItemId(usize),
    #[error("referenced command #{0} exists, but does not have charge item ID info")]
    BackrefCmdNoChargeItemId(usize),
}
impl HExecError {
    pub(crate) fn get_api_code(&self) -> String {
        match self {
            // Fits
            HExecError::FitNotFoundPrimary(_) => "FIT-001",
            HExecError::FitNotFoundSecondary(_) => "FIT-002",
            HExecError::FitCharacterNotFound(_) => "FIT-003",
            HExecError::FitShipNotFound(_) => "FIT-004",
            HExecError::FitStanceNotFound(_) => "FIT-005",
            HExecError::FitNotInFleet(_) => "FIT-006",
            HExecError::FitNotInThisFleet(_) => "FIT-007",
            HExecError::FitAlreadyInThisFleet(_) => "FIT-008",
            // Fleets
            HExecError::FleetNotFoundPrimary(_) => "FLT-001",
            HExecError::FleetNotFoundSecondary(_) => "FLT-002",
            // Items
            HExecError::ItemNotFoundPrimary(_) => "ITM-001",
            HExecError::ItemNotFoundSecondary(_) => "ITM-002",
            HExecError::ItemKindMismatch(_) => "ITM-003",
            HExecError::SkillIdCollision(_) => "SKL-001",
            HExecError::MutationNotSet(_) => "MUT-001",
            HExecError::ChargeNotSet(_) => "NCH-001",
            HExecError::UnremovableAutocharge => "ACH-001",
            HExecError::InvalidFighterCount(_) => "FTR-001",
            HExecError::ProjecteeCantTakeProjs(_) => "PRJ-001",
            HExecError::ProjectionNotFound(_) => "PRJ-002",
            HExecError::ProjectionAlreadyExists(_) => "PRJ-003",
            // Backreferences
            HExecError::BackrefCmdNotFound(_) => "REF-001",
            HExecError::BackrefCmdNoFitId(_) => "REF-002",
            HExecError::BackrefCmdNoFleetId(_) => "REF-003",
            HExecError::BackrefCmdNoItemId(_) => "REF-004",
            HExecError::BackrefCmdNoChargeItemId(_) => "REF-005",
        }
        .to_string()
    }
}
