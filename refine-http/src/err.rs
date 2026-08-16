use axum::{
    Json,
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub(crate) enum ApiError {
    #[error("{}", .0.body_text())]
    Query(QueryRejection),
    #[error("{}", .0.body_text())]
    Json(JsonRejection),
    #[error(transparent)]
    BatchParse(ApiErrorIndexed<serde_json::Error>),
    #[error(transparent)]
    BackrefRender(ApiErrorIndexed<rs::err::BackrefRenderError>),
    #[error("failed to read request body")]
    RequestRead(#[source] axum::Error),
    #[error("failed to process request body: {0}")]
    RequestTooLarge(String),
    // Source-related
    #[error("\"{0}\" cannot be used as a source alias")]
    PathSrcParseOnAdd(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error("alias \"{0}\" not found")]
    PathSrcParseMisc(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error(transparent)]
    PathSrcNotFound(#[from] rs::src::err::GetSrcError),
    #[error("alias \"{0}\" not found")]
    BodySrcParse(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error("EVE data handler not found for requested format \"{0}\"")]
    EdhNotFound(String),
    #[error("EVE data handler initialization failed")]
    EdhInit(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    SrcAdd(#[from] rs::src::err::AddSrcError),
    #[error(transparent)]
    SrcRemove(#[from] rs::src::err::RemoveSrcError),
    // Solar system-related
    #[error(transparent)]
    PathSolParse(#[from] rs::err::ParseSolarSystemIdError),
    #[error(transparent)]
    PathSolNotFound(#[from] rs::err::GetSolError),
    #[error(transparent)]
    SolAdd(#[from] rs::err::AddSolError),
    #[error(transparent)]
    SolChange(ApiErrorIndexed<rs::err::ChangeSolEnumError>),
    #[error(transparent)]
    SolRemove(#[from] rs::err::RemoveSolError),
    #[error(transparent)]
    SolSrcSwitch(#[from] rs::err::SolSwitchSrcError),
    // Fleet-related
    #[error(transparent)]
    PathFleetParse(#[from] rs::err::ParseFleetIdError),
    #[error(transparent)]
    PathFleetNotFound(#[from] rs::err::GetFleetError),
    #[error(transparent)]
    FleetAdd(#[from] rs::err::FleetAddError),
    #[error(transparent)]
    FleetChange(#[from] rs::err::FleetChangeError),
    // Fit-related
    #[error(transparent)]
    PathFitParse(#[from] rs::err::ParseFitIdError),
    #[error(transparent)]
    PathFitNotFound(#[from] rs::err::GetFitError),
    #[error(transparent)]
    FitAdd(#[from] rs::err::FitAddError),
    #[error(transparent)]
    FitChange(ApiErrorIndexed<rs::err::FitCtlCmdError>),
    // Item-related
    #[error(transparent)]
    PathItemParse(#[from] rs::err::ParseItemIdError),
    #[error(transparent)]
    PathItemNotFound(#[from] rs::err::GetItemError),
    #[error(transparent)]
    ItemAdd(#[from] rs::err::ItemAddError),
    #[error(transparent)]
    ItemChange(#[from] rs::err::ItemCtlError),
    #[error(transparent)]
    ItemRemove(#[from] rs::err::RemoveItemError),
}

#[derive(Serialize)]
struct ApiErrorResponse {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmd_index: Option<usize>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Codes & messages
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ApiError {
    fn get_codes(&self) -> (StatusCode, &'static str) {
        match self {
            Self::Query(..) => (StatusCode::BAD_REQUEST, "PRM-001"),
            Self::Json(err) => match err {
                // Failure to read body is not really a JSON error, so make it behave like the
                // regular request read error
                JsonRejection::BytesRejection(..) => (StatusCode::BAD_REQUEST, "REQ-001"),
                _ => (StatusCode::BAD_REQUEST, "JSN-001"),
            },
            Self::BatchParse(..) => (StatusCode::BAD_REQUEST, "JSN-002"),
            Self::BackrefRender(..) => (StatusCode::BAD_REQUEST, "BRF-001"),
            Self::RequestRead(..) => (StatusCode::BAD_REQUEST, "REQ-001"),
            Self::RequestTooLarge(..) => (StatusCode::PAYLOAD_TOO_LARGE, "REQ-002"),
            ////////////////////////////////////////////////////////////////////////////////////////
            // Source-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSrcParseOnAdd(..) => (StatusCode::FORBIDDEN, "SRC-004"),
            Self::PathSrcParseMisc(..) => (StatusCode::NOT_FOUND, "SRC-003"),
            Self::PathSrcNotFound(err) => match err {
                rs::src::err::GetSrcError::SrcNotFound(..) => (StatusCode::NOT_FOUND, "SRC-001"),
                rs::src::err::GetSrcError::DefaultNotDefined => (StatusCode::NOT_FOUND, "SRC-002"),
            },
            Self::BodySrcParse(..) => (StatusCode::BAD_REQUEST, "SRC-005"),
            Self::EdhNotFound(..) => (StatusCode::BAD_REQUEST, "EDH-001"),
            Self::EdhInit(..) => (StatusCode::BAD_REQUEST, "EDH-002"),
            Self::SrcAdd(err) => match err {
                rs::src::err::AddSrcError::SrcAliasNotAvailable(..) => (StatusCode::FORBIDDEN, "SRC-006"),
                rs::src::err::AddSrcError::SrcInit(..) => (StatusCode::UNPROCESSABLE_ENTITY, "SNT-001"),
            },
            Self::SrcRemove(err) => match err {
                rs::src::err::RemoveSrcError::SrcNotFound(..) => (StatusCode::NOT_FOUND, "SRC-007"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Solar system-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSolParse(..) => (StatusCode::NOT_FOUND, "SOL-002"),
            Self::PathSolNotFound(err) => match err {
                rs::err::GetSolError::SolNotFound(..) => (StatusCode::NOT_FOUND, "SOL-001"),
            },
            Self::SolAdd(err) => match err {
                rs::err::AddSolError::SrcGet(..) => (StatusCode::BAD_REQUEST, "SOL-003"),
            },
            Self::SolChange(err) => match &err.error {
                // Fleets
                rs::err::ChangeSolEnumError::FleetAdd(rs::err::FleetAddError::FitAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "FLT-003")
                }
                rs::err::ChangeSolEnumError::FleetChange(err_l2) => match err_l2 {
                    rs::err::FleetGetFleetChangeError::FleetGet(..) => (StatusCode::BAD_REQUEST, "FLT-001"),
                    rs::err::FleetGetFleetChangeError::FitAdd(..) => (StatusCode::BAD_REQUEST, "FLT-004"),
                    rs::err::FleetGetFleetChangeError::FitRemove(..) => (StatusCode::BAD_REQUEST, "FLT-005"),
                },
                rs::err::ChangeSolEnumError::FleetRemove(rs::err::FleetGetFleetRemoveError::FleetGet(_)) => {
                    (StatusCode::BAD_REQUEST, "FLT-001")
                }
                // Fits
                rs::err::ChangeSolEnumError::FitAdd(rs::err::FitAddError::FleetSet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-003")
                }
                rs::err::ChangeSolEnumError::FitChange(err_l2) => match err_l2 {
                    rs::err::FitGetFitChangeError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetFitChangeError::FleetSet(..) => (StatusCode::BAD_REQUEST, "FIT-004"),
                },
                rs::err::ChangeSolEnumError::FitRemove(rs::err::FitGetFitRemoveError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item
                rs::err::ChangeSolEnumError::ItemRemove(err_l2) => match err_l2 {
                    rs::err::ItemGetItemRemoveError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetItemRemoveError::ItemRemove(
                        rs::err::core::RemoveItemError::UnremovableAutocharge,
                    ) => (StatusCode::BAD_REQUEST, "ACH-002"),
                },
                // Item - autocharge
                rs::err::ChangeSolEnumError::AutochargeChange(err_l2) => match err_l2 {
                    rs::err::ItemGetAutochargeChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetAutochargeChangeError::ItemIsNotAutocharge(..) => {
                        (StatusCode::BAD_REQUEST, "ACH-001")
                    }
                },
                // Item - booster
                rs::err::ChangeSolEnumError::BoosterAdd(rs::err::FitGetBoosterAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::BoosterChange(err_l2) => match err_l2 {
                    rs::err::ItemGetBoosterChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetBoosterChangeError::ItemIsNotBooster(..) => (StatusCode::BAD_REQUEST, "BST-001"),
                },
                // Item - character
                rs::err::ChangeSolEnumError::CharacterSet(rs::err::GetFitSetCharacterError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::CharacterChange(err_l2) => match err_l2 {
                    rs::err::ChangeCharacterError::CharacterChangeViaFit(err_l3) => match err_l3 {
                        rs::err::GetFitChangeCharacterError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::GetFitChangeCharacterError::FitNoCharacter(..) => (StatusCode::BAD_REQUEST, "CHR-002"),
                    },
                    rs::err::ChangeCharacterError::CharacterChangeViaItem(
                        rs::err::GetItemChangeCharacterError::ItemGet(err_l3),
                    ) => match err_l3 {
                        rs::err::core::GetCharacterError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                        rs::err::core::GetCharacterError::ItemIsNotCharacter(..) => {
                            (StatusCode::BAD_REQUEST, "CHR-001")
                        }
                    },
                },
                rs::err::ChangeSolEnumError::CharacterUnset(rs::err::GetFitUnsetCharacterError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - charge
                rs::err::ChangeSolEnumError::ChargeChange(err_l2) => match err_l2 {
                    rs::err::ItemGetChargeChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetChargeChangeError::ItemIsNotCharge(..) => (StatusCode::BAD_REQUEST, "CHG-001"),
                },
                // Item - drone
                rs::err::ChangeSolEnumError::DroneAdd(err_l2) => match err_l2 {
                    rs::err::GetFitAddDroneError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddDroneError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-002"),
                },
                rs::err::ChangeSolEnumError::DroneChange(err_l2) => match err_l2 {
                    rs::err::GetItemChangeDroneError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeDroneError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::GetItemChangeDroneError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::GetItemChangeDroneError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::GetItemChangeDroneError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                // Item - fighter
                rs::err::ChangeSolEnumError::FighterAdd(err_l2) => match err_l2 {
                    rs::err::GetFitAddFighterError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddFighterError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-002"),
                },
                rs::err::ChangeSolEnumError::FighterChange(err_l2) => match err_l2 {
                    rs::err::GetItemChangeFighterError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeFighterError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::GetItemChangeFighterError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::GetItemChangeFighterError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                // Item - fit-wide effect
                rs::err::ChangeSolEnumError::FwEffectAdd(rs::err::FitGetFwEffectAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::FwEffectChange(err_l2) => match err_l2 {
                    rs::err::ItemGetFwEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetFwEffectChangeError::ItemIsNotFwEffect(..) => (StatusCode::BAD_REQUEST, "FWE-001"),
                },
                // Item - implant
                rs::err::ChangeSolEnumError::ImplantAdd(rs::err::FitGetImplantAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::ImplantChange(err_l2) => match err_l2 {
                    rs::err::ItemGetImplantChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetImplantChangeError::ItemIsNotImplant(..) => (StatusCode::BAD_REQUEST, "IMP-001"),
                },
                // Item - module
                rs::err::ChangeSolEnumError::ModuleAdd(err_l2) => match err_l2 {
                    rs::err::GetFitAddModuleError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddModuleError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-002"),
                },
                rs::err::ChangeSolEnumError::ModuleChange(err_l2) => match err_l2 {
                    rs::err::GetItemChangeModuleError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeModuleError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::GetItemChangeModuleError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::GetItemChangeModuleError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::GetItemChangeModuleError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                // Item - projected effect
                rs::err::ChangeSolEnumError::ProjEffectAdd(rs::err::ProjEffectAddError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "PJE-002")
                }
                rs::err::ChangeSolEnumError::ProjEffectChange(err_l2) => match err_l2 {
                    rs::err::ItemGetProjEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetProjEffectChangeError::ItemIsNotProjEffect(..) => {
                        (StatusCode::BAD_REQUEST, "PJE-001")
                    }
                    rs::err::ItemGetProjEffectChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "PJE-003"),
                    rs::err::ItemGetProjEffectChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "PJE-004"),
                },
                // Item - rig
                rs::err::ChangeSolEnumError::RigAdd(rs::err::FitGetRigAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::RigChange(err_l2) => match err_l2 {
                    rs::err::ItemGetRigChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetRigChangeError::ItemIsNotRig(..) => (StatusCode::BAD_REQUEST, "RIG-001"),
                },
                // Item - service
                rs::err::ChangeSolEnumError::ServiceAdd(rs::err::FitGetServiceAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::ServiceChange(err_l2) => match err_l2 {
                    rs::err::ItemGetServiceChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetServiceChangeError::ItemIsNotService(..) => (StatusCode::BAD_REQUEST, "SVC-001"),
                },
                // Item - ship
                rs::err::ChangeSolEnumError::ShipSet(rs::err::GetFitSetShipError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::ShipChange(err_l2) => match err_l2 {
                    rs::err::ChangeShipError::ShipChangeViaFit(err_l3) => match err_l3 {
                        rs::err::GetFitChangeShipError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::GetFitChangeShipError::FitNoShip(..) => (StatusCode::BAD_REQUEST, "SHP-002"),
                    },
                    rs::err::ChangeShipError::ShipChangeViaItem(rs::err::GetItemChangeShipError::ItemGet(err_l3)) => {
                        match err_l3 {
                            rs::err::core::GetShipError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                            rs::err::core::GetShipError::ItemIsNotShip(..) => (StatusCode::BAD_REQUEST, "SHP-001"),
                        }
                    }
                },
                rs::err::ChangeSolEnumError::ShipUnset(rs::err::GetFitUnsetShipError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - skill
                rs::err::ChangeSolEnumError::SkillAdd(err_l2) => match err_l2 {
                    rs::err::FitGetSkillAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetSkillAddError::SkillAdd(rs::err::core::AddSkillError::SkillIdCollision(..)) => {
                        (StatusCode::BAD_REQUEST, "SKL-002")
                    }
                },
                rs::err::ChangeSolEnumError::SkillChange(err_l2) => match err_l2 {
                    rs::err::ItemGetSkillChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetSkillChangeError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::ItemGetSkillChangeError::TypeIdSet(
                        rs::err::core::SetSkillTypeIdError::SkillIdCollision(..),
                    ) => (StatusCode::BAD_REQUEST, "SKL-003"),
                },
                // Item - stance
                rs::err::ChangeSolEnumError::StanceSet(rs::err::GetFitSetStanceError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::StanceChange(err_l2) => match err_l2 {
                    rs::err::ChangeStanceError::StanceChangeViaFit(err_l3) => match err_l3 {
                        rs::err::GetFitChangeStanceError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::GetFitChangeStanceError::FitNoStance(..) => (StatusCode::BAD_REQUEST, "STC-002"),
                    },
                    rs::err::ChangeStanceError::StanceChangeViaItem(rs::err::GetItemChangeStanceError::ItemGet(
                        err_l3,
                    )) => match err_l3 {
                        rs::err::core::GetStanceError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                        rs::err::core::GetStanceError::ItemIsNotStance(..) => (StatusCode::BAD_REQUEST, "STC-001"),
                    },
                },
                rs::err::ChangeSolEnumError::StanceUnset(rs::err::GetFitUnsetStanceError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - subsystem
                rs::err::ChangeSolEnumError::SubsystemAdd(rs::err::FitGetSubsystemAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::SubsystemChange(err_l2) => match err_l2 {
                    rs::err::ItemGetSubsystemChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetSubsystemChangeError::ItemIsNotSubsystem(..) => {
                        (StatusCode::BAD_REQUEST, "SUB-001")
                    }
                },
                // Item - system-wide effect
                rs::err::ChangeSolEnumError::SwEffectChange(err_l2) => match err_l2 {
                    rs::err::ItemGetSwEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetSwEffectChangeError::ItemIsNotSwEffect(..) => (StatusCode::BAD_REQUEST, "SWE-001"),
                },
            },
            Self::SolRemove(err) => match err {
                rs::err::RemoveSolError::SolNotFound(..) => (StatusCode::NOT_FOUND, "SOL-004"),
            },
            Self::SolSrcSwitch(err) => match err {
                rs::err::SolSwitchSrcError::SrcGet(..) => (StatusCode::BAD_REQUEST, "SOL-005"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fleet-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFleetParse(..) => (StatusCode::NOT_FOUND, "FLT-002"),
            Self::PathFleetNotFound(..) => (StatusCode::NOT_FOUND, "FLT-001"),
            Self::FleetAdd(err) => match err {
                rs::err::FleetAddError::FitAdd(..) => (StatusCode::BAD_REQUEST, "FLT-003"),
            },
            Self::FleetChange(err) => match err {
                rs::err::FleetChangeError::FitAdd(..) => (StatusCode::BAD_REQUEST, "FLT-004"),
                rs::err::FleetChangeError::FitRemove(..) => (StatusCode::BAD_REQUEST, "FLT-005"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fit-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFitParse(..) => (StatusCode::NOT_FOUND, "FIT-002"),
            Self::PathFitNotFound(..) => (StatusCode::NOT_FOUND, "FIT-001"),
            Self::FitAdd(err) => match err {
                rs::err::FitAddError::FleetSet(..) => (StatusCode::BAD_REQUEST, "FIT-003"),
            },
            Self::FitChange(err) => match &err.error {
                // Fit
                rs::err::FitCtlCmdError::FitChange(rs::err::FitChangeError::FleetSet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-004")
                }
                // Item
                rs::err::FitCtlCmdError::ItemRemove(err_l2) => match err_l2 {
                    rs::err::ItemGetItemRemoveError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetItemRemoveError::ItemRemove(
                        rs::err::core::RemoveItemError::UnremovableAutocharge,
                    ) => (StatusCode::BAD_REQUEST, "ACH-002"),
                },
                // Item - autocharge
                rs::err::FitCtlCmdError::AutochargeChange(err_l2) => match err_l2 {
                    rs::err::ItemGetAutochargeChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetAutochargeChangeError::ItemIsNotAutocharge(..) => {
                        (StatusCode::BAD_REQUEST, "ACH-001")
                    }
                },
                // Item - booster
                rs::err::FitCtlCmdError::BoosterChange(err_l2) => match err_l2 {
                    rs::err::ItemGetBoosterChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetBoosterChangeError::ItemIsNotBooster(..) => (StatusCode::BAD_REQUEST, "BST-001"),
                },
                // Item - character
                rs::err::FitCtlCmdError::CharacterChange(rs::err::FitChangeCharacterError::FitNoCharacter(..)) => {
                    (StatusCode::BAD_REQUEST, "CHR-002")
                }
                // Item - charge
                rs::err::FitCtlCmdError::ChargeChange(err_l2) => match err_l2 {
                    rs::err::ItemGetChargeChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetChargeChangeError::ItemIsNotCharge(..) => (StatusCode::BAD_REQUEST, "CHG-001"),
                },
                // Item - drone
                rs::err::FitCtlCmdError::DroneAdd(rs::err::FitAddDroneError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "DRN-002")
                }
                rs::err::FitCtlCmdError::DroneChange(err_l2) => match err_l2 {
                    rs::err::GetItemChangeDroneError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeDroneError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::GetItemChangeDroneError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::GetItemChangeDroneError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::GetItemChangeDroneError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                // Item - fighter
                rs::err::FitCtlCmdError::FighterAdd(rs::err::FitAddFighterError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "FTR-002")
                }
                rs::err::FitCtlCmdError::FighterChange(err_l2) => match err_l2 {
                    rs::err::GetItemChangeFighterError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeFighterError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::GetItemChangeFighterError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::GetItemChangeFighterError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                // Item - fit-wide effect
                rs::err::FitCtlCmdError::FwEffectChange(err_l2) => match err_l2 {
                    rs::err::ItemGetFwEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetFwEffectChangeError::ItemIsNotFwEffect(..) => (StatusCode::BAD_REQUEST, "FWE-001"),
                },
                // Item - implant
                rs::err::FitCtlCmdError::ImplantChange(err_l2) => match err_l2 {
                    rs::err::ItemGetImplantChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetImplantChangeError::ItemIsNotImplant(..) => (StatusCode::BAD_REQUEST, "IMP-001"),
                },
                // Item - module
                rs::err::FitCtlCmdError::ModuleAdd(rs::err::FitAddModuleError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "MOD-002")
                }
                rs::err::FitCtlCmdError::ModuleChange(err_l2) => match err_l2 {
                    rs::err::GetItemChangeModuleError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeModuleError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::GetItemChangeModuleError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::GetItemChangeModuleError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::GetItemChangeModuleError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                // Item - rig
                rs::err::FitCtlCmdError::RigChange(err_l2) => match err_l2 {
                    rs::err::ItemGetRigChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetRigChangeError::ItemIsNotRig(..) => (StatusCode::BAD_REQUEST, "RIG-001"),
                },
                // Item - service
                rs::err::FitCtlCmdError::ServiceChange(err_l2) => match err_l2 {
                    rs::err::ItemGetServiceChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetServiceChangeError::ItemIsNotService(..) => (StatusCode::BAD_REQUEST, "SVC-001"),
                },
                // Item - ship
                rs::err::FitCtlCmdError::ShipChange(rs::err::FitChangeShipError::FitNoShip(..)) => {
                    (StatusCode::BAD_REQUEST, "SHP-002")
                }
                // Item - skill
                rs::err::FitCtlCmdError::SkillAdd(rs::err::SkillAddError::SkillAdd(
                    rs::err::core::AddSkillError::SkillIdCollision(..),
                )) => (StatusCode::BAD_REQUEST, "SKL-002"),
                rs::err::FitCtlCmdError::SkillChange(err_l2) => match err_l2 {
                    rs::err::ItemGetSkillChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetSkillChangeError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::ItemGetSkillChangeError::TypeIdSet(
                        rs::err::core::SetSkillTypeIdError::SkillIdCollision(..),
                    ) => (StatusCode::BAD_REQUEST, "SKL-003"),
                },
                // Item - stance
                rs::err::FitCtlCmdError::StanceChange(rs::err::FitChangeStanceError::FitNoStance(..)) => {
                    (StatusCode::BAD_REQUEST, "STC-002")
                }
                // Item - subsystem
                rs::err::FitCtlCmdError::SubsystemChange(err_l2) => match err_l2 {
                    rs::err::ItemGetSubsystemChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetSubsystemChangeError::ItemIsNotSubsystem(..) => {
                        (StatusCode::BAD_REQUEST, "SUB-001")
                    }
                },
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Item-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathItemParse(..) => (StatusCode::NOT_FOUND, "ITM-002"),
            Self::PathItemNotFound(..) => (StatusCode::NOT_FOUND, "ITM-001"),
            Self::ItemAdd(err_l1) => match err_l1 {
                rs::err::ItemAddError::Booster(rs::err::FitGetBoosterAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddError::Character(rs::err::GetFitSetCharacterError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddError::Drone(err_l2) => match err_l2 {
                    rs::err::GetFitAddDroneError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddDroneError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-002"),
                },
                rs::err::ItemAddError::Fighter(err_l2) => match err_l2 {
                    rs::err::GetFitAddFighterError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddFighterError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-002"),
                },
                rs::err::ItemAddError::FwEffect(rs::err::FitGetFwEffectAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddError::Implant(rs::err::FitGetImplantAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddError::Module(err_l2) => match err_l2 {
                    rs::err::GetFitAddModuleError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddModuleError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-002"),
                },
                rs::err::ItemAddError::ProjEffect(rs::err::ProjEffectAddError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "PJE-002")
                }
                rs::err::ItemAddError::Rig(rs::err::FitGetRigAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddError::Service(rs::err::FitGetServiceAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddError::Ship(rs::err::GetFitSetShipError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddError::Skill(err_l2) => match err_l2 {
                    rs::err::FitGetSkillAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetSkillAddError::SkillAdd(rs::err::core::AddSkillError::SkillIdCollision(..)) => {
                        (StatusCode::BAD_REQUEST, "SKL-002")
                    }
                },
                rs::err::ItemAddError::Stance(rs::err::GetFitSetStanceError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddError::Subsystem(rs::err::FitGetSubsystemAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
            },
            Self::ItemChange(err_l1) => match err_l1 {
                rs::err::ItemCtlError::Autocharge(rs::err::AutochargeChangeError::ItemIsNotAutocharge(..)) => {
                    (StatusCode::BAD_REQUEST, "ACH-001")
                }
                rs::err::ItemCtlError::Booster(rs::err::BoosterChangeError::ItemIsNotBooster(..)) => {
                    (StatusCode::BAD_REQUEST, "BST-001")
                }
                rs::err::ItemCtlError::Character(rs::err::ItemChangeCharacterError::ItemIsNotCharacter(..)) => {
                    (StatusCode::BAD_REQUEST, "CHR-001")
                }
                rs::err::ItemCtlError::Charge(rs::err::ChargeChangeError::ItemIsNotCharge(..)) => {
                    (StatusCode::BAD_REQUEST, "CHG-001")
                }
                rs::err::ItemCtlError::Drone(err_l2) => match err_l2 {
                    rs::err::ItemChangeDroneError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::ItemChangeDroneError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::ItemChangeDroneError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::ItemChangeDroneError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                rs::err::ItemCtlError::Fighter(err_l2) => match err_l2 {
                    rs::err::ItemChangeFighterError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::ItemChangeFighterError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::ItemChangeFighterError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                rs::err::ItemCtlError::FwEffect(rs::err::FwEffectChangeError::ItemIsNotFwEffect(_)) => {
                    (StatusCode::BAD_REQUEST, "FWE-001")
                }
                rs::err::ItemCtlError::Implant(rs::err::ImplantChangeError::ItemIsNotImplant(..)) => {
                    (StatusCode::BAD_REQUEST, "IMP-001")
                }
                rs::err::ItemCtlError::Module(err_l2) => match err_l2 {
                    rs::err::ItemChangeModuleError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::ItemChangeModuleError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::ItemChangeModuleError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::ItemChangeModuleError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                rs::err::ItemCtlError::ProjEffect(err_l2) => match err_l2 {
                    rs::err::ProjEffectChangeError::ItemIsNotProjEffect(..) => (StatusCode::BAD_REQUEST, "PJE-001"),
                    rs::err::ProjEffectChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "PJE-003"),
                    rs::err::ProjEffectChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "PJE-004"),
                },
                rs::err::ItemCtlError::Rig(rs::err::RigChangeError::ItemIsNotRig(..)) => {
                    (StatusCode::BAD_REQUEST, "ITM-001")
                }
                rs::err::ItemCtlError::Service(rs::err::ServiceChangeError::ItemIsNotService(..)) => {
                    (StatusCode::BAD_REQUEST, "SVC-001")
                }
                rs::err::ItemCtlError::Ship(rs::err::ItemChangeShipError::ItemIsNotShip(..)) => {
                    (StatusCode::BAD_REQUEST, "SHP-001")
                }
                rs::err::ItemCtlError::Skill(err_l2) => match err_l2 {
                    rs::err::SkillChangeError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::SkillChangeError::TypeIdSet(rs::err::core::SetSkillTypeIdError::SkillIdCollision(..)) => {
                        (StatusCode::BAD_REQUEST, "SKL-003")
                    }
                },
                rs::err::ItemCtlError::Stance(rs::err::ItemChangeStanceError::ItemIsNotStance(..)) => {
                    (StatusCode::BAD_REQUEST, "STC-001")
                }
                rs::err::ItemCtlError::Subsystem(rs::err::SubsystemChangeError::ItemIsNotSubsystem(..)) => {
                    (StatusCode::BAD_REQUEST, "ITM-001")
                }
                rs::err::ItemCtlError::SwEffect(rs::err::SwEffectChangeError::ItemIsNotSwEffect(_)) => {
                    (StatusCode::BAD_REQUEST, "SWE-001")
                }
            },
            Self::ItemRemove(rs::err::RemoveItemError(rs::err::ItemRemoveError::ItemRemove(
                rs::err::core::RemoveItemError::UnremovableAutocharge,
            ))) => (StatusCode::FORBIDDEN, "ACH-002"),
        }
    }
    fn get_cmd_index(&self) -> Option<usize> {
        match self {
            Self::BatchParse(err) => Some(err.index),
            Self::BackrefRender(err) => Some(err.index),
            Self::SolChange(err) => Some(err.index),
            Self::FitChange(err) => Some(err.index),
            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
// Carries extra info about source of a command batch execution failure
#[derive(Debug)]
pub(crate) struct ApiErrorIndexed<E> {
    pub(crate) index: usize,
    pub(crate) error: E,
}
impl<E> ApiErrorIndexed<E> {
    pub(crate) fn new(index: usize, error: E) -> Self {
        Self { index, error }
    }
}
impl<E> std::fmt::Display for ApiErrorIndexed<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, f)
    }
}
impl<E> std::error::Error for ApiErrorIndexed<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<QueryRejection> for ApiError {
    fn from(err: QueryRejection) -> Self {
        Self::Query(err)
    }
}
impl From<JsonRejection> for ApiError {
    fn from(err: JsonRejection) -> Self {
        Self::Json(err)
    }
}
impl From<rs::err::ChangeSolError> for ApiError {
    fn from(err: rs::err::ChangeSolError) -> Self {
        match err {
            rs::err::ChangeSolError::CtlRender(index, inner) => {
                Self::BackrefRender(ApiErrorIndexed { index, error: inner })
            }
            rs::err::ChangeSolError::CtlExec(index, inner) => Self::SolChange(ApiErrorIndexed { index, error: inner }),
        }
    }
}
impl From<rs::err::CtlFitChangeError> for ApiError {
    fn from(err: rs::err::CtlFitChangeError) -> Self {
        match err {
            rs::err::CtlFitChangeError::CtlRender(index, inner) => {
                Self::BackrefRender(ApiErrorIndexed { index, error: inner })
            }
            rs::err::CtlFitChangeError::CtlExec(index, inner) => {
                Self::FitChange(ApiErrorIndexed { index, error: inner })
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (http_code, api_code) = self.get_codes();
        let cmd_index = self.get_cmd_index();
        let payload = ApiErrorResponse {
            code: api_code,
            message: std::error::Report::new(&self).to_string(),
            cmd_index,
        };
        (http_code, Json(payload)).into_response()
    }
}
