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
    #[error("failed to read request body")]
    RequestRead(#[source] axum::Error),
    #[error("failed to process request body: {0}")]
    RequestTooLarge(String),
    // Batch-related, but not specific to any entities
    #[error(transparent)]
    BatchParse(ApiErrorIndexed<serde_json::Error>),
    #[error(transparent)]
    BatchBackrefResolve(ApiErrorIndexed<rs::err::BrResolveError>),
    // Source-related
    #[error("\"{0}\" cannot be used as a source alias")]
    PathSrcParseOnAdd(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error("alias \"{0}\" not found")]
    PathSrcParseMisc(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error(transparent)]
    PathSrcNotFound(#[from] rs::src::err::SrcGetError),
    #[error("alias \"{0}\" not found")]
    BodySrcParse(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error("EVE data handler not found for requested format \"{0}\"")]
    EdhNotFound(String),
    #[error("EVE data handler initialization failed")]
    EdhInit(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    SrcAdd(#[from] rs::src::err::SrcAddError),
    #[error(transparent)]
    SrcRemove(#[from] rs::src::err::SrcRemoveError),
    // Solar system-related
    #[error(transparent)]
    PathSolParse(#[from] rs::err::ParseSolarSystemIdError),
    #[error(transparent)]
    PathSolNotFound(#[from] rs::err::SolGetError),
    #[error(transparent)]
    SolAdd(#[from] rs::err::SolAddError),
    #[error(transparent)]
    SolChange(#[from] rs::err::SolChangeEnumSolInfoError),
    #[error(transparent)]
    SolRemove(#[from] rs::err::SolRemoveError),
    #[error(transparent)]
    SolSrcSwitch(#[from] rs::err::SolSwitchSrcError),
    #[error(transparent)]
    SolBatch(ApiErrorIndexed<rs::err::SolChangeEnumError>),
    // Fleet-related
    #[error(transparent)]
    PathFleetParse(#[from] rs::err::ParseFleetIdError),
    #[error(transparent)]
    PathFleetNotFound(#[from] rs::err::FleetGetError),
    #[error(transparent)]
    FleetAdd(#[from] rs::err::FleetAddError),
    #[error(transparent)]
    FleetChange(#[from] rs::err::FleetChangeError),
    // Fit-related
    #[error(transparent)]
    PathFitParse(#[from] rs::err::ParseFitIdError),
    #[error(transparent)]
    PathFitNotFound(#[from] rs::err::FitGetError),
    #[error(transparent)]
    FitAdd(#[from] rs::err::FitAddError),
    #[error(transparent)]
    FitChange(#[from] rs::err::FitChangeEnumFitInfoError),
    #[error(transparent)]
    FitBatch(ApiErrorIndexed<rs::err::FitChangeEnumError>),
    // Item-related
    #[error(transparent)]
    PathItemParse(#[from] rs::err::ParseItemIdError),
    #[error(transparent)]
    PathItemNotFound(#[from] rs::err::ItemGetError),
    #[error(transparent)]
    ItemAdd(#[from] rs::err::ItemAddEnumError),
    #[error(transparent)]
    ItemChange(#[from] rs::err::ItemChangeEnumError),
    #[error(transparent)]
    ItemRemove(#[from] rs::err::ItemRemoveError),
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
            Self::BatchBackrefResolve(..) => (StatusCode::BAD_REQUEST, "BRF-001"),
            Self::RequestRead(..) => (StatusCode::BAD_REQUEST, "REQ-001"),
            Self::RequestTooLarge(..) => (StatusCode::PAYLOAD_TOO_LARGE, "REQ-002"),
            ////////////////////////////////////////////////////////////////////////////////////////
            // Source-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSrcParseOnAdd(..) => (StatusCode::FORBIDDEN, "SRC-004"),
            Self::PathSrcParseMisc(..) => (StatusCode::NOT_FOUND, "SRC-003"),
            Self::PathSrcNotFound(err) => match err {
                rs::src::err::SrcGetError::SrcNotFound(..) => (StatusCode::NOT_FOUND, "SRC-001"),
                rs::src::err::SrcGetError::DefaultNotDefined => (StatusCode::NOT_FOUND, "SRC-002"),
            },
            Self::BodySrcParse(..) => (StatusCode::BAD_REQUEST, "SRC-005"),
            Self::EdhNotFound(..) => (StatusCode::BAD_REQUEST, "EDH-001"),
            Self::EdhInit(..) => (StatusCode::BAD_REQUEST, "EDH-002"),
            Self::SrcAdd(err) => match err {
                rs::src::err::SrcAddError::SrcAliasNotAvailable(..) => (StatusCode::FORBIDDEN, "SRC-006"),
                rs::src::err::SrcAddError::SrcInit(..) => (StatusCode::UNPROCESSABLE_ENTITY, "SNT-001"),
            },
            Self::SrcRemove(err) => match err {
                rs::src::err::SrcRemoveError::SrcNotFound(..) => (StatusCode::NOT_FOUND, "SRC-007"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Solar system-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSolParse(..) => (StatusCode::NOT_FOUND, "SOL-002"),
            Self::PathSolNotFound(err) => match err {
                rs::err::SolGetError::SolNotFound(..) => (StatusCode::NOT_FOUND, "SOL-001"),
            },
            Self::SolAdd(err) => match err {
                rs::err::SolAddError::SrcGet(..) => (StatusCode::BAD_REQUEST, "SOL-003"),
            },
            // TODO: change to expose proper error codes
            Self::SolChange(..) => (StatusCode::BAD_REQUEST, "AAA-000"),
            Self::SolBatch(err) => match &err.error {
                // Fleets
                rs::err::SolChangeEnumError::FleetAdd(rs::err::FleetAddError::FitAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "FLT-003")
                }
                rs::err::SolChangeEnumError::FleetChange(err_l2) => match err_l2 {
                    rs::err::FleetGetFleetChangeError::FleetGet(..) => (StatusCode::BAD_REQUEST, "FLT-001"),
                    rs::err::FleetGetFleetChangeError::FitAdd(..) => (StatusCode::BAD_REQUEST, "FLT-004"),
                    rs::err::FleetGetFleetChangeError::FitRemove(..) => (StatusCode::BAD_REQUEST, "FLT-005"),
                },
                rs::err::SolChangeEnumError::FleetRemove(rs::err::FleetGetFleetRemoveError::FleetGet(_)) => {
                    (StatusCode::BAD_REQUEST, "FLT-001")
                }
                // Fits
                rs::err::SolChangeEnumError::FitAdd(rs::err::FitAddError::FleetSet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-003")
                }
                rs::err::SolChangeEnumError::FitChange(err_l2) => match err_l2 {
                    rs::err::FitGetFitChangeError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetFitChangeError::FleetSet(..) => (StatusCode::BAD_REQUEST, "FIT-004"),
                },
                rs::err::SolChangeEnumError::FitRemove(rs::err::FitGetFitRemoveError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item
                rs::err::SolChangeEnumError::ItemRemove(err_l2) => match err_l2 {
                    rs::err::ItemGetItemRemoveError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetItemRemoveError::ItemRemove(
                        rs::err::core::RemoveItemError::UnremovableAutocharge,
                    ) => (StatusCode::BAD_REQUEST, "ACH-002"),
                },
                // Item - autocharge
                rs::err::SolChangeEnumError::AutochargeChange(err_l2) => match err_l2 {
                    rs::err::ItemGetAutochargeChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetAutochargeChangeError::ItemIsNotAutocharge(..) => {
                        (StatusCode::BAD_REQUEST, "ACH-001")
                    }
                },
                // Item - booster
                rs::err::SolChangeEnumError::BoosterAdd(rs::err::FitGetBoosterAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::SolChangeEnumError::BoosterChange(err_l2) => match err_l2 {
                    rs::err::ItemGetBoosterChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetBoosterChangeError::ItemIsNotBooster(..) => (StatusCode::BAD_REQUEST, "BST-001"),
                },
                // Item - character
                rs::err::SolChangeEnumError::CharacterSet(rs::err::FitGetCharacterSetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::SolChangeEnumError::CharacterChange(err_l2) => match err_l2 {
                    rs::err::CharacterChangeError::ViaFit(err_l3) => match err_l3 {
                        rs::err::FitGetCharacterChangeError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::FitGetCharacterChangeError::FitNoCharacter(..) => (StatusCode::BAD_REQUEST, "CHR-002"),
                    },
                    rs::err::CharacterChangeError::ViaItem(rs::err::ItemGetCharacterChangeError::ItemGet(err_l3)) => {
                        match err_l3 {
                            rs::err::core::GetCharacterError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                            rs::err::core::GetCharacterError::ItemIsNotCharacter(..) => {
                                (StatusCode::BAD_REQUEST, "CHR-001")
                            }
                        }
                    }
                },
                rs::err::SolChangeEnumError::CharacterUnset(rs::err::FitGetCharacterUnsetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - charge
                rs::err::SolChangeEnumError::ChargeChange(err_l2) => match err_l2 {
                    rs::err::ItemGetChargeChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetChargeChangeError::ItemIsNotCharge(..) => (StatusCode::BAD_REQUEST, "CHG-001"),
                },
                // Item - drone
                rs::err::SolChangeEnumError::DroneAdd(err_l2) => match err_l2 {
                    rs::err::FitGetDroneAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetDroneAddError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-002"),
                },
                rs::err::SolChangeEnumError::DroneChange(err_l2) => match err_l2 {
                    rs::err::ItemGetDroneChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetDroneChangeError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::ItemGetDroneChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::ItemGetDroneChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::ItemGetDroneChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                // Item - fighter
                rs::err::SolChangeEnumError::FighterAdd(err_l2) => match err_l2 {
                    rs::err::FitGetFighterAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetFighterAddError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-002"),
                },
                rs::err::SolChangeEnumError::FighterChange(err_l2) => match err_l2 {
                    rs::err::ItemGetFighterChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetFighterChangeError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::ItemGetFighterChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::ItemGetFighterChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                // Item - fit-wide effect
                rs::err::SolChangeEnumError::FwEffectAdd(rs::err::FitGetFwEffectAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::SolChangeEnumError::FwEffectChange(err_l2) => match err_l2 {
                    rs::err::ItemGetFwEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetFwEffectChangeError::ItemIsNotFwEffect(..) => (StatusCode::BAD_REQUEST, "FWE-001"),
                },
                // Item - implant
                rs::err::SolChangeEnumError::ImplantAdd(rs::err::FitGetImplantAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::SolChangeEnumError::ImplantChange(err_l2) => match err_l2 {
                    rs::err::ItemGetImplantChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetImplantChangeError::ItemIsNotImplant(..) => (StatusCode::BAD_REQUEST, "IMP-001"),
                },
                // Item - module
                rs::err::SolChangeEnumError::ModuleAdd(err_l2) => match err_l2 {
                    rs::err::FitGetModuleAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetModuleAddError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-002"),
                },
                rs::err::SolChangeEnumError::ModuleChange(err_l2) => match err_l2 {
                    rs::err::ItemGetModuleChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetModuleChangeError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::ItemGetModuleChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::ItemGetModuleChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::ItemGetModuleChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                // Item - projected effect
                rs::err::SolChangeEnumError::ProjEffectAdd(rs::err::ProjEffectAddError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "PJE-002")
                }
                rs::err::SolChangeEnumError::ProjEffectChange(err_l2) => match err_l2 {
                    rs::err::ItemGetProjEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetProjEffectChangeError::ItemIsNotProjEffect(..) => {
                        (StatusCode::BAD_REQUEST, "PJE-001")
                    }
                    rs::err::ItemGetProjEffectChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "PJE-003"),
                    rs::err::ItemGetProjEffectChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "PJE-004"),
                },
                // Item - rig
                rs::err::SolChangeEnumError::RigAdd(rs::err::FitGetRigAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::SolChangeEnumError::RigChange(err_l2) => match err_l2 {
                    rs::err::ItemGetRigChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetRigChangeError::ItemIsNotRig(..) => (StatusCode::BAD_REQUEST, "RIG-001"),
                },
                // Item - service
                rs::err::SolChangeEnumError::ServiceAdd(rs::err::FitGetServiceAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::SolChangeEnumError::ServiceChange(err_l2) => match err_l2 {
                    rs::err::ItemGetServiceChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetServiceChangeError::ItemIsNotService(..) => (StatusCode::BAD_REQUEST, "SVC-001"),
                },
                // Item - ship
                rs::err::SolChangeEnumError::ShipSet(rs::err::FitGetShipSetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::SolChangeEnumError::ShipChange(err_l2) => match err_l2 {
                    rs::err::ShipChangeError::ViaFit(err_l3) => match err_l3 {
                        rs::err::FitGetShipChangeError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::FitGetShipChangeError::FitNoShip(..) => (StatusCode::BAD_REQUEST, "SHP-002"),
                    },
                    rs::err::ShipChangeError::ViaItem(rs::err::ItemGetShipChangeError::ItemGet(err_l3)) => match err_l3
                    {
                        rs::err::core::GetShipError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                        rs::err::core::GetShipError::ItemIsNotShip(..) => (StatusCode::BAD_REQUEST, "SHP-001"),
                    },
                },
                rs::err::SolChangeEnumError::ShipUnset(rs::err::FitGetShipUnsetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - skill
                rs::err::SolChangeEnumError::SkillAdd(err_l2) => match err_l2 {
                    rs::err::FitGetSkillAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetSkillAddError::SkillAdd(rs::err::core::AddSkillError::SkillIdCollision(..)) => {
                        (StatusCode::BAD_REQUEST, "SKL-002")
                    }
                },
                rs::err::SolChangeEnumError::SkillChange(err_l2) => match err_l2 {
                    rs::err::ItemGetSkillChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetSkillChangeError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::ItemGetSkillChangeError::TypeIdSet(
                        rs::err::core::SetSkillTypeIdError::SkillIdCollision(..),
                    ) => (StatusCode::BAD_REQUEST, "SKL-003"),
                },
                // Item - stance
                rs::err::SolChangeEnumError::StanceSet(rs::err::FitGetStanceSetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::SolChangeEnumError::StanceChange(err_l2) => match err_l2 {
                    rs::err::StanceChangeError::ViaFit(err_l3) => match err_l3 {
                        rs::err::FitGetStanceChangeError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::FitGetStanceChangeError::FitNoStance(..) => (StatusCode::BAD_REQUEST, "STC-002"),
                    },
                    rs::err::StanceChangeError::ViaItem(rs::err::ItemGetStanceChangeError::ItemGet(err_l3)) => {
                        match err_l3 {
                            rs::err::core::GetStanceError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                            rs::err::core::GetStanceError::ItemIsNotStance(..) => (StatusCode::BAD_REQUEST, "STC-001"),
                        }
                    }
                },
                rs::err::SolChangeEnumError::StanceUnset(rs::err::FitGetStanceUnsetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - subsystem
                rs::err::SolChangeEnumError::SubsystemAdd(rs::err::FitGetSubsystemAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::SolChangeEnumError::SubsystemChange(err_l2) => match err_l2 {
                    rs::err::ItemGetSubsystemChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetSubsystemChangeError::ItemIsNotSubsystem(..) => {
                        (StatusCode::BAD_REQUEST, "SUB-001")
                    }
                },
                // Item - system-wide effect
                rs::err::SolChangeEnumError::SwEffectChange(err_l2) => match err_l2 {
                    rs::err::ItemGetSwEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetSwEffectChangeError::ItemIsNotSwEffect(..) => (StatusCode::BAD_REQUEST, "SWE-001"),
                },
            },
            Self::SolRemove(err) => match err {
                rs::err::SolRemoveError::SolNotFound(..) => (StatusCode::NOT_FOUND, "SOL-004"),
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
            // TODO: change to expose proper error codes
            Self::FitChange(..) => (StatusCode::BAD_REQUEST, "AAA-000"),
            Self::FitBatch(err) => match &err.error {
                // Fit
                rs::err::FitChangeEnumError::FitChange(rs::err::FitChangeError::FleetSet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-004")
                }
                // Item
                rs::err::FitChangeEnumError::ItemRemove(err_l2) => match err_l2 {
                    rs::err::ItemGetItemRemoveError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetItemRemoveError::ItemRemove(
                        rs::err::core::RemoveItemError::UnremovableAutocharge,
                    ) => (StatusCode::BAD_REQUEST, "ACH-002"),
                },
                // Item - autocharge
                rs::err::FitChangeEnumError::AutochargeChange(err_l2) => match err_l2 {
                    rs::err::ItemGetAutochargeChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetAutochargeChangeError::ItemIsNotAutocharge(..) => {
                        (StatusCode::BAD_REQUEST, "ACH-001")
                    }
                },
                // Item - booster
                rs::err::FitChangeEnumError::BoosterChange(err_l2) => match err_l2 {
                    rs::err::ItemGetBoosterChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetBoosterChangeError::ItemIsNotBooster(..) => (StatusCode::BAD_REQUEST, "BST-001"),
                },
                // Item - character
                rs::err::FitChangeEnumError::CharacterChange(rs::err::FitCharacterChangeError::FitNoCharacter(..)) => {
                    (StatusCode::BAD_REQUEST, "CHR-002")
                }
                // Item - charge
                rs::err::FitChangeEnumError::ChargeChange(err_l2) => match err_l2 {
                    rs::err::ItemGetChargeChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetChargeChangeError::ItemIsNotCharge(..) => (StatusCode::BAD_REQUEST, "CHG-001"),
                },
                // Item - drone
                rs::err::FitChangeEnumError::DroneAdd(rs::err::DroneAddError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "DRN-002")
                }
                rs::err::FitChangeEnumError::DroneChange(err_l2) => match err_l2 {
                    rs::err::ItemGetDroneChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetDroneChangeError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::ItemGetDroneChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::ItemGetDroneChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::ItemGetDroneChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                // Item - fighter
                rs::err::FitChangeEnumError::FighterAdd(rs::err::FighterAddError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "FTR-002")
                }
                rs::err::FitChangeEnumError::FighterChange(err_l2) => match err_l2 {
                    rs::err::ItemGetFighterChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetFighterChangeError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::ItemGetFighterChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::ItemGetFighterChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                // Item - fit-wide effect
                rs::err::FitChangeEnumError::FwEffectChange(err_l2) => match err_l2 {
                    rs::err::ItemGetFwEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetFwEffectChangeError::ItemIsNotFwEffect(..) => (StatusCode::BAD_REQUEST, "FWE-001"),
                },
                // Item - implant
                rs::err::FitChangeEnumError::ImplantChange(err_l2) => match err_l2 {
                    rs::err::ItemGetImplantChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetImplantChangeError::ItemIsNotImplant(..) => (StatusCode::BAD_REQUEST, "IMP-001"),
                },
                // Item - module
                rs::err::FitChangeEnumError::ModuleAdd(rs::err::ModuleAddError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "MOD-002")
                }
                rs::err::FitChangeEnumError::ModuleChange(err_l2) => match err_l2 {
                    rs::err::ItemGetModuleChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetModuleChangeError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::ItemGetModuleChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::ItemGetModuleChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::ItemGetModuleChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                // Item - rig
                rs::err::FitChangeEnumError::RigChange(err_l2) => match err_l2 {
                    rs::err::ItemGetRigChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetRigChangeError::ItemIsNotRig(..) => (StatusCode::BAD_REQUEST, "RIG-001"),
                },
                // Item - service
                rs::err::FitChangeEnumError::ServiceChange(err_l2) => match err_l2 {
                    rs::err::ItemGetServiceChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetServiceChangeError::ItemIsNotService(..) => (StatusCode::BAD_REQUEST, "SVC-001"),
                },
                // Item - ship
                rs::err::FitChangeEnumError::ShipChange(rs::err::FitShipChangeError::FitNoShip(..)) => {
                    (StatusCode::BAD_REQUEST, "SHP-002")
                }
                // Item - skill
                rs::err::FitChangeEnumError::SkillAdd(rs::err::SkillAddError::SkillAdd(
                    rs::err::core::AddSkillError::SkillIdCollision(..),
                )) => (StatusCode::BAD_REQUEST, "SKL-002"),
                rs::err::FitChangeEnumError::SkillChange(err_l2) => match err_l2 {
                    rs::err::ItemGetSkillChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::ItemGetSkillChangeError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::ItemGetSkillChangeError::TypeIdSet(
                        rs::err::core::SetSkillTypeIdError::SkillIdCollision(..),
                    ) => (StatusCode::BAD_REQUEST, "SKL-003"),
                },
                // Item - stance
                rs::err::FitChangeEnumError::StanceChange(rs::err::FitStanceChangeError::FitNoStance(..)) => {
                    (StatusCode::BAD_REQUEST, "STC-002")
                }
                // Item - subsystem
                rs::err::FitChangeEnumError::SubsystemChange(err_l2) => match err_l2 {
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
                rs::err::ItemAddEnumError::Booster(rs::err::FitGetBoosterAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddEnumError::Character(rs::err::FitGetCharacterSetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddEnumError::Drone(err_l2) => match err_l2 {
                    rs::err::FitGetDroneAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetDroneAddError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-002"),
                },
                rs::err::ItemAddEnumError::Fighter(err_l2) => match err_l2 {
                    rs::err::FitGetFighterAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetFighterAddError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-002"),
                },
                rs::err::ItemAddEnumError::FwEffect(rs::err::FitGetFwEffectAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddEnumError::Implant(rs::err::FitGetImplantAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddEnumError::Module(err_l2) => match err_l2 {
                    rs::err::FitGetModuleAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetModuleAddError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-002"),
                },
                rs::err::ItemAddEnumError::ProjEffect(rs::err::ProjEffectAddError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "PJE-002")
                }
                rs::err::ItemAddEnumError::Rig(rs::err::FitGetRigAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddEnumError::Service(rs::err::FitGetServiceAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddEnumError::Ship(rs::err::FitGetShipSetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddEnumError::Skill(err_l2) => match err_l2 {
                    rs::err::FitGetSkillAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::FitGetSkillAddError::SkillAdd(rs::err::core::AddSkillError::SkillIdCollision(..)) => {
                        (StatusCode::BAD_REQUEST, "SKL-002")
                    }
                },
                rs::err::ItemAddEnumError::Stance(rs::err::FitGetStanceSetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ItemAddEnumError::Subsystem(rs::err::FitGetSubsystemAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
            },
            Self::ItemChange(err_l1) => match err_l1 {
                rs::err::ItemChangeEnumError::Autocharge(rs::err::AutochargeChangeError::ItemIsNotAutocharge(..)) => {
                    (StatusCode::BAD_REQUEST, "ACH-001")
                }
                rs::err::ItemChangeEnumError::Booster(rs::err::BoosterChangeError::ItemIsNotBooster(..)) => {
                    (StatusCode::BAD_REQUEST, "BST-001")
                }
                rs::err::ItemChangeEnumError::Character(rs::err::ItemCharacterChangeError::ItemIsNotCharacter(..)) => {
                    (StatusCode::BAD_REQUEST, "CHR-001")
                }
                rs::err::ItemChangeEnumError::Charge(rs::err::ChargeChangeError::ItemIsNotCharge(..)) => {
                    (StatusCode::BAD_REQUEST, "CHG-001")
                }
                rs::err::ItemChangeEnumError::Drone(err_l2) => match err_l2 {
                    rs::err::DroneChangeError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::DroneChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::DroneChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::DroneChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                rs::err::ItemChangeEnumError::Fighter(err_l2) => match err_l2 {
                    rs::err::FighterChangeError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::FighterChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::FighterChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                rs::err::ItemChangeEnumError::FwEffect(rs::err::FwEffectChangeError::ItemIsNotFwEffect(_)) => {
                    (StatusCode::BAD_REQUEST, "FWE-001")
                }
                rs::err::ItemChangeEnumError::Implant(rs::err::ImplantChangeError::ItemIsNotImplant(..)) => {
                    (StatusCode::BAD_REQUEST, "IMP-001")
                }
                rs::err::ItemChangeEnumError::Module(err_l2) => match err_l2 {
                    rs::err::ModuleChangeError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::ModuleChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::ModuleChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::ModuleChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                rs::err::ItemChangeEnumError::ProjEffect(err_l2) => match err_l2 {
                    rs::err::ProjEffectChangeError::ItemIsNotProjEffect(..) => (StatusCode::BAD_REQUEST, "PJE-001"),
                    rs::err::ProjEffectChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "PJE-003"),
                    rs::err::ProjEffectChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "PJE-004"),
                },
                rs::err::ItemChangeEnumError::Rig(rs::err::RigChangeError::ItemIsNotRig(..)) => {
                    (StatusCode::BAD_REQUEST, "ITM-001")
                }
                rs::err::ItemChangeEnumError::Service(rs::err::ServiceChangeError::ItemIsNotService(..)) => {
                    (StatusCode::BAD_REQUEST, "SVC-001")
                }
                rs::err::ItemChangeEnumError::Ship(rs::err::ItemShipChangeError::ItemIsNotShip(..)) => {
                    (StatusCode::BAD_REQUEST, "SHP-001")
                }
                rs::err::ItemChangeEnumError::Skill(err_l2) => match err_l2 {
                    rs::err::SkillChangeError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::SkillChangeError::TypeIdSet(rs::err::core::SetSkillTypeIdError::SkillIdCollision(..)) => {
                        (StatusCode::BAD_REQUEST, "SKL-003")
                    }
                },
                rs::err::ItemChangeEnumError::Stance(rs::err::ItemStanceChangeError::ItemIsNotStance(..)) => {
                    (StatusCode::BAD_REQUEST, "STC-001")
                }
                rs::err::ItemChangeEnumError::Subsystem(rs::err::SubsystemChangeError::ItemIsNotSubsystem(..)) => {
                    (StatusCode::BAD_REQUEST, "ITM-001")
                }
                rs::err::ItemChangeEnumError::SwEffect(rs::err::SwEffectChangeError::ItemIsNotSwEffect(_)) => {
                    (StatusCode::BAD_REQUEST, "SWE-001")
                }
            },
            Self::ItemRemove(rs::err::ItemRemoveError::ItemRemove(
                rs::err::core::RemoveItemError::UnremovableAutocharge,
            )) => (StatusCode::FORBIDDEN, "ACH-002"),
        }
    }
    fn get_cmd_index(&self) -> Option<usize> {
        match self {
            Self::BatchParse(err) => Some(err.index),
            Self::BatchBackrefResolve(err) => Some(err.index),
            Self::SolBatch(err) => Some(err.index),
            Self::FitBatch(err) => Some(err.index),
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
impl From<rs::err::SolHybridBatchError> for ApiError {
    fn from(err: rs::err::SolHybridBatchError) -> Self {
        match err {
            rs::err::SolHybridBatchError::BrResolve(index, inner) => {
                Self::BatchBackrefResolve(ApiErrorIndexed { index, error: inner })
            }
            rs::err::SolHybridBatchError::CtlExec(index, inner) => {
                Self::SolBatch(ApiErrorIndexed { index, error: inner })
            }
        }
    }
}
impl From<rs::err::FitChangeBatchError> for ApiError {
    fn from(err: rs::err::FitChangeBatchError) -> Self {
        match err {
            rs::err::FitChangeBatchError::BrResolve(index, inner) => {
                Self::BatchBackrefResolve(ApiErrorIndexed { index, error: inner })
            }
            rs::err::FitChangeBatchError::CtlExec(index, inner) => {
                Self::FitBatch(ApiErrorIndexed { index, error: inner })
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
