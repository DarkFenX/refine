#[derive(Debug)]
pub(crate) enum HExecError {
    // Fits
    FitNotFoundPrimary(rc::err::basic::FitFoundError),
    FitNotFoundSecondary(rc::err::basic::FitFoundError),
    FitCharacterNotFound(rc::SolFitId),
    FitShipNotFound(rc::SolFitId),
    FitStanceNotFound(rc::SolFitId),
    FitIsNotInFleet(rc::err::basic::FitFleetAssignedError),
    FitIsNotInThisFleet(rc::SolFleetId, rc::SolFitId),
    // Fleets
    FleetNotFoundPrimary(rc::err::basic::FleetFoundError),
    FleetNotFoundSecondary(rc::err::basic::FleetFoundError),
    // Items
    ItemNotFoundPrimary(rc::err::basic::ItemFoundError),
    ItemNotFoundSecondary(rc::err::basic::ItemFoundError),
    ItemKindMismatch(rc::err::basic::ItemKindMatchError),
    ChargeNotSet(rc::err::basic::ChargeFoundError),
    UnremovableAutocharge(rc::err::basic::ItemKindRemoveError),
    InvalidSkillLevel(rc::err::basic::SkillLevelError),
    NotBoosterSideEffect(rc::err::basic::SideEffectError),
    ModuleSlotTaken(rc::err::basic::OrderedSlotError),
    ProjecteeCantTakeProjs(rc::err::basic::ItemReceiveProjError),
    ProjectionNotFound(rc::err::basic::ProjFoundError),
    ProjectionAlreadyExists(rc::err::basic::ProjNotFoundError),
}
impl std::error::Error for HExecError {}
impl std::fmt::Display for HExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Fits
            HExecError::FitNotFoundPrimary(e) => write!(f, "{e}"),
            HExecError::FitNotFoundSecondary(e) => write!(f, "{e}"),
            HExecError::FitCharacterNotFound(fit_id) => write!(f, "fit {fit_id} has no character set"),
            HExecError::FitShipNotFound(fit_id) => write!(f, "fit {fit_id} has no ship set"),
            HExecError::FitStanceNotFound(fit_id) => write!(f, "fit {fit_id} has no stance set"),
            HExecError::FitIsNotInFleet(e) => write!(f, "{e}"),
            HExecError::FitIsNotInThisFleet(fleet_id, fit_id) => write!(f, "fit {fit_id} is not in fleet {fleet_id}"),
            // Fleets
            HExecError::FleetNotFoundPrimary(e) => write!(f, "{e}"),
            HExecError::FleetNotFoundSecondary(e) => write!(f, "{e}"),
            // Items
            HExecError::ItemNotFoundPrimary(e) => write!(f, "{e}"),
            HExecError::ItemNotFoundSecondary(e) => write!(f, "{e}"),
            HExecError::ItemKindMismatch(e) => write!(f, "{e}"),
            HExecError::ChargeNotSet(e) => write!(f, "{e}"),
            HExecError::UnremovableAutocharge(e) => write!(f, "{e}"),
            HExecError::InvalidSkillLevel(e) => write!(f, "{e}"),
            HExecError::NotBoosterSideEffect(e) => write!(f, "{e}"),
            HExecError::ModuleSlotTaken(e) => write!(f, "{e}"),
            HExecError::ProjecteeCantTakeProjs(e) => write!(f, "{e}"),
            HExecError::ProjectionNotFound(e) => write!(f, "{e}"),
            HExecError::ProjectionAlreadyExists(e) => write!(f, "{e}"),
        }
    }
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
            HExecError::FitIsNotInFleet(_) => "EXC-007",
            HExecError::FitIsNotInThisFleet(_, _) => "EXC-008",
            // Fleets
            HExecError::FleetNotFoundPrimary(_) => "EXC-010",
            HExecError::FleetNotFoundSecondary(_) => "EXC-011",
            // Items
            HExecError::ItemNotFoundPrimary(_) => "EXC-013",
            HExecError::ItemNotFoundSecondary(_) => "EXC-014",
            HExecError::ItemKindMismatch(_) => "EXC-015",
            HExecError::ChargeNotSet(_) => "EXC-016",
            HExecError::UnremovableAutocharge(_) => "EXC-017",
            HExecError::InvalidSkillLevel(_) => "EXC-018",
            HExecError::NotBoosterSideEffect(_) => "EXC-019",
            HExecError::ModuleSlotTaken(_) => "EXC-020",
            HExecError::ProjecteeCantTakeProjs(_) => "EXC-021",
            HExecError::ProjectionNotFound(_) => "EXC-022",
            HExecError::ProjectionAlreadyExists(_) => "EXC-023",
        }
        .to_string()
    }
}
