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
    #[error("{1}")]
    BatchParseFailed(usize, String),
    #[error("{1}")]
    BackrefRenderFailed(usize, #[source] rs::err::BackrefRenderError),
    #[error("failed to read request body: {0}")]
    RequestReadFailed(String),
    #[error("failed to process request body: {0}")]
    RequestTooLarge(String),
    // Source-related
    #[error("\"{0}\" cannot be used as a source alias: {1}")]
    PathSrcParseFailedOnAdd(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error("alias \"{0}\" not found")]
    PathSrcParseFailedMisc(#[source] rs::src::err::SrcAliasPruneInitError),
    #[error("{0}")]
    PathSrcNotFound(#[from] rs::src::err::GetSrcError),
    #[error("alias \"{0}\" not found")]
    BodySrcParseFailed(String, #[source] rs::src::err::SrcAliasPruneInitError),
    #[error("{0}")]
    SrcAddFailed(#[from] rs::src::err::AddSrcError),
    #[error("{0}")]
    SrcRemoveFailed(#[from] rs::src::err::RemoveSrcError),
    // Solar system-related
    #[error("{0}")]
    PathSolParseFailed(#[from] rs::err::ParseSolarSystemIdError),
    #[error("{0}")]
    PathSolNotFound(#[from] rs::err::GetSolError),
    #[error("{0}")]
    SolAddFailed(#[from] rs::err::AddSolError),
    #[error("{1}")]
    SolChangeFailed(usize, #[source] rs::err::ChangeSolEnumError),
    #[error("{0}")]
    SolRemoveFailed(#[from] rs::err::RemoveSolError),
    #[error("{0}")]
    SolSrcSwitch(#[from] rs::err::SolSwitchSrcError),
    // Fleet-related
    #[error("{0}")]
    PathFleetParseFailed(#[from] rs::err::ParseFleetIdError),
    #[error("{0}")]
    PathFleetNotFound(#[from] rs::err::GetFleetError),
    #[error("{0}")]
    FleetAddFailed(#[from] rs::err::AddFleetError),
    #[error("{0}")]
    FleetChangeFailed(#[from] rs::err::ChangeFleetError),
    // Fit-related
    #[error("{0}")]
    PathFitParseFailed(#[from] rs::err::ParseFitIdError),
    #[error("{0}")]
    PathFitNotFound(#[from] rs::err::GetFitError),
    #[error("{0}")]
    FitAddFailed(#[from] rs::err::AddFitError),
    #[error("{1}")]
    FitChangeFailed(usize, #[source] rs::err::ChangeFitEnumError),
    // Item-related
    #[error("{0}")]
    PathItemParseFailed(#[from] rs::err::ParseItemIdError),
    #[error("{0}")]
    PathItemNotFound(#[from] rs::err::GetItemError),
    #[error("{0}")]
    ItemAddFailed(#[from] rs::err::AddItemEnumError),
    #[error("{0}")]
    ItemChangeFailed(#[from] rs::err::ChangeItemEnumError),
    #[error("{0}")]
    ItemRemoveFailed(#[from] rs::err::RemoveItemError),
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
            Self::BatchParseFailed(..) => (StatusCode::BAD_REQUEST, "JSN-002"),
            Self::BackrefRenderFailed(..) => (StatusCode::BAD_REQUEST, "BRF-001"),
            Self::RequestReadFailed(..) => (StatusCode::BAD_REQUEST, "REQ-001"),
            Self::RequestTooLarge(..) => (StatusCode::PAYLOAD_TOO_LARGE, "REQ-002"),
            ////////////////////////////////////////////////////////////////////////////////////////
            // Source-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSrcParseFailedOnAdd(..) => (StatusCode::FORBIDDEN, "SRC-004"),
            Self::PathSrcParseFailedMisc(..) => (StatusCode::NOT_FOUND, "SRC-003"),
            Self::PathSrcNotFound(err) => match err {
                rs::src::err::GetSrcError::SrcNotFound(..) => (StatusCode::NOT_FOUND, "SRC-001"),
                rs::src::err::GetSrcError::DefaultNotDefined => (StatusCode::NOT_FOUND, "SRC-002"),
            },
            Self::BodySrcParseFailed(..) => (StatusCode::BAD_REQUEST, "SRC-005"),
            Self::SrcAddFailed(err) => match err {
                rs::src::err::AddSrcError::SrcAliasNotAvailable(..) => (StatusCode::FORBIDDEN, "SRC-006"),
                rs::src::err::AddSrcError::EdhInitFailed(..) => (StatusCode::BAD_REQUEST, "EDH-001"),
                rs::src::err::AddSrcError::SrcInitFailed(..) => (StatusCode::UNPROCESSABLE_ENTITY, "SNT-001"),
            },
            Self::SrcRemoveFailed(err) => match err {
                rs::src::err::RemoveSrcError::SrcNotFound(..) => (StatusCode::NOT_FOUND, "SRC-007"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Solar system-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSolParseFailed(..) => (StatusCode::NOT_FOUND, "SOL-002"),
            Self::PathSolNotFound(err) => match err {
                rs::err::GetSolError::SolNotFound(..) => (StatusCode::NOT_FOUND, "SOL-001"),
            },
            Self::SolAddFailed(err) => match err {
                rs::err::AddSolError::GetSrcFailed(..) => (StatusCode::BAD_REQUEST, "SOL-003"),
            },
            Self::SolChangeFailed(_, err_l1) => match err_l1 {
                // Fleets
                rs::err::ChangeSolEnumError::FleetAddFailed(rs::err::AddFleetError::FitAddFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FLT-003")
                }
                rs::err::ChangeSolEnumError::FleetChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetFleetChangeFleetError::FleetGetFailed(..) => (StatusCode::BAD_REQUEST, "FLT-001"),
                    rs::err::GetFleetChangeFleetError::FitAddFailed(..) => (StatusCode::BAD_REQUEST, "FLT-004"),
                    rs::err::GetFleetChangeFleetError::FitRemoveFailed(..) => (StatusCode::BAD_REQUEST, "FLT-005"),
                },
                rs::err::ChangeSolEnumError::FleetRemoveFailed(rs::err::GetFleetRemoveFleetError::FleetGetFailed(
                    _,
                )) => (StatusCode::BAD_REQUEST, "FLT-001"),
                // Fits
                rs::err::ChangeSolEnumError::FitAddFailed(rs::err::AddFitError::FleetSetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-003")
                }
                rs::err::ChangeSolEnumError::FitChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetFitChangeFitError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitChangeFitError::FleetSetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-004"),
                },
                rs::err::ChangeSolEnumError::FitRemoveFailed(rs::err::GetFitRemoveFitError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item
                rs::err::ChangeSolEnumError::ItemRemoveFailed(err_l2) => match err_l2 {
                    rs::err::GetItemRemoveItemError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemRemoveItemError::ItemRemoveFailed(
                        rs::err::core::RemoveItemError::UnremovableAutocharge,
                    ) => (StatusCode::BAD_REQUEST, "ACH-002"),
                },
                // Item - autocharge
                rs::err::ChangeSolEnumError::AutochargeChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeAutochargeError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeAutochargeError::ItemIsNotAutocharge(..) => {
                        (StatusCode::BAD_REQUEST, "ACH-001")
                    }
                },
                // Item - booster
                rs::err::ChangeSolEnumError::BoosterAddFailed(rs::err::GetFitAddBoosterError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::BoosterChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeBoosterError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeBoosterError::ItemIsNotBooster(..) => (StatusCode::BAD_REQUEST, "BST-001"),
                },
                // Item - character
                rs::err::ChangeSolEnumError::CharacterSetFailed(rs::err::GetFitSetCharacterError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::CharacterChangeFailed(err_l2) => match err_l2 {
                    rs::err::ChangeCharacterError::CharacterChangeViaFitFailed(err_l3) => match err_l3 {
                        rs::err::GetFitChangeCharacterError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::GetFitChangeCharacterError::FitNoCharacter(..) => (StatusCode::BAD_REQUEST, "CHR-002"),
                    },
                    rs::err::ChangeCharacterError::CharacterChangeViaItemFailed(
                        rs::err::GetItemChangeCharacterError::ItemGetFailed(err_l3),
                    ) => match err_l3 {
                        rs::err::core::GetCharacterError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                        rs::err::core::GetCharacterError::ItemIsNotCharacter(..) => {
                            (StatusCode::BAD_REQUEST, "CHR-001")
                        }
                    },
                },
                rs::err::ChangeSolEnumError::CharacterUnsetFailed(
                    rs::err::GetFitUnsetCharacterError::FitGetFailed(..),
                ) => (StatusCode::BAD_REQUEST, "FIT-001"),
                // Item - charge
                rs::err::ChangeSolEnumError::ChargeChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeChargeError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeChargeError::ItemIsNotCharge(..) => (StatusCode::BAD_REQUEST, "CHG-001"),
                },
                // Item - drone
                rs::err::ChangeSolEnumError::DroneAddFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddDroneError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddDroneError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "DRN-002"),
                },
                rs::err::ChangeSolEnumError::DroneChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeDroneError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeDroneError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::GetItemChangeDroneError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::GetItemChangeDroneError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::GetItemChangeDroneError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                // Item - fighter
                rs::err::ChangeSolEnumError::FighterAddFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddFighterError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddFighterError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "FTR-002"),
                },
                rs::err::ChangeSolEnumError::FighterChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeFighterError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeFighterError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::GetItemChangeFighterError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::GetItemChangeFighterError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                // Item - fit-wide effect
                rs::err::ChangeSolEnumError::FwEffectAddFailed(rs::err::GetFitAddFwEffectError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::FwEffectChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeFwEffectError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeFwEffectError::ItemIsNotFwEffect(..) => (StatusCode::BAD_REQUEST, "FWE-001"),
                },
                // Item - implant
                rs::err::ChangeSolEnumError::ImplantAddFailed(rs::err::GetFitAddImplantError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::ImplantChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeImplantError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeImplantError::ItemIsNotImplant(..) => (StatusCode::BAD_REQUEST, "IMP-001"),
                },
                // Item - module
                rs::err::ChangeSolEnumError::ModuleAddFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddModuleError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddModuleError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "MOD-002"),
                },
                rs::err::ChangeSolEnumError::ModuleChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeModuleError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeModuleError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::GetItemChangeModuleError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::GetItemChangeModuleError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::GetItemChangeModuleError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                // Item - projected effect
                rs::err::ChangeSolEnumError::ProjEffectAddFailed(rs::err::AddProjEffectError::ProjAddFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "PJE-002")
                }
                rs::err::ChangeSolEnumError::ProjEffectChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeProjEffectError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeProjEffectError::ItemIsNotProjEffect(..) => {
                        (StatusCode::BAD_REQUEST, "PJE-001")
                    }
                    rs::err::GetItemChangeProjEffectError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "PJE-003"),
                    rs::err::GetItemChangeProjEffectError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "PJE-004"),
                },
                // Item - rig
                rs::err::ChangeSolEnumError::RigAddFailed(rs::err::GetFitAddRigError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::RigChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeRigError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeRigError::ItemIsNotRig(..) => (StatusCode::BAD_REQUEST, "RIG-001"),
                },
                // Item - service
                rs::err::ChangeSolEnumError::ServiceAddFailed(rs::err::GetFitAddServiceError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::ServiceChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeServiceError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeServiceError::ItemIsNotService(..) => (StatusCode::BAD_REQUEST, "SVC-001"),
                },
                // Item - ship
                rs::err::ChangeSolEnumError::ShipSetFailed(rs::err::GetFitSetShipError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::ShipChangeFailed(err_l2) => match err_l2 {
                    rs::err::ChangeShipError::ShipChangeViaFitFailed(err_l3) => match err_l3 {
                        rs::err::GetFitChangeShipError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::GetFitChangeShipError::FitNoShip(..) => (StatusCode::BAD_REQUEST, "SHP-002"),
                    },
                    rs::err::ChangeShipError::ShipChangeViaItemFailed(
                        rs::err::GetItemChangeShipError::ItemGetFailed(err_l3),
                    ) => match err_l3 {
                        rs::err::core::GetShipError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                        rs::err::core::GetShipError::ItemIsNotShip(..) => (StatusCode::BAD_REQUEST, "SHP-001"),
                    },
                },
                rs::err::ChangeSolEnumError::ShipUnsetFailed(rs::err::GetFitUnsetShipError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - skill
                rs::err::ChangeSolEnumError::SkillAddFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddSkillError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddSkillError::SkillAddFailed(rs::err::core::AddSkillError::SkillIdCollision(
                        ..,
                    )) => (StatusCode::BAD_REQUEST, "SKL-002"),
                },
                rs::err::ChangeSolEnumError::SkillChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeSkillError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeSkillError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::GetItemChangeSkillError::TypeIdSetFailed(
                        rs::err::core::SetSkillTypeIdError::SkillIdCollision(..),
                    ) => (StatusCode::BAD_REQUEST, "SKL-003"),
                },
                // Item - stance
                rs::err::ChangeSolEnumError::StanceSetFailed(rs::err::GetFitSetStanceError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::StanceChangeFailed(err_l2) => match err_l2 {
                    rs::err::ChangeStanceError::StanceChangeViaFitFailed(err_l3) => match err_l3 {
                        rs::err::GetFitChangeStanceError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::GetFitChangeStanceError::FitNoStance(..) => (StatusCode::BAD_REQUEST, "STC-002"),
                    },
                    rs::err::ChangeStanceError::StanceChangeViaItemFailed(
                        rs::err::GetItemChangeStanceError::ItemGetFailed(err_l3),
                    ) => match err_l3 {
                        rs::err::core::GetStanceError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                        rs::err::core::GetStanceError::ItemIsNotStance(..) => (StatusCode::BAD_REQUEST, "STC-001"),
                    },
                },
                rs::err::ChangeSolEnumError::StanceUnsetFailed(rs::err::GetFitUnsetStanceError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - subsystem
                rs::err::ChangeSolEnumError::SubsystemAddFailed(rs::err::GetFitAddSubsystemError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::SubsystemChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeSubsystemError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeSubsystemError::ItemIsNotSubsystem(..) => {
                        (StatusCode::BAD_REQUEST, "SUB-001")
                    }
                },
                // Item - system-wide effect
                rs::err::ChangeSolEnumError::SwEffectChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeSwEffectError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeSwEffectError::ItemIsNotSwEffect(..) => (StatusCode::BAD_REQUEST, "SWE-001"),
                },
            },
            Self::SolRemoveFailed(err) => match err {
                rs::err::RemoveSolError::SolNotFound(..) => (StatusCode::NOT_FOUND, "SOL-004"),
            },
            Self::SolSrcSwitch(err) => match err {
                rs::err::SolSwitchSrcError::SrcGetFailed(..) => (StatusCode::BAD_REQUEST, "SOL-005"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fleet-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFleetParseFailed(..) => (StatusCode::NOT_FOUND, "FLT-002"),
            Self::PathFleetNotFound(..) => (StatusCode::NOT_FOUND, "FLT-001"),
            Self::FleetAddFailed(err) => match err {
                rs::err::AddFleetError::FitAddFailed(..) => (StatusCode::BAD_REQUEST, "FLT-003"),
            },
            Self::FleetChangeFailed(rs::err::ChangeFleetError(err)) => match err {
                rs::err::FleetChangeFleetError::FitAddFailed(..) => (StatusCode::BAD_REQUEST, "FLT-004"),
                rs::err::FleetChangeFleetError::FitRemoveFailed(..) => (StatusCode::BAD_REQUEST, "FLT-005"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fit-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFitParseFailed(..) => (StatusCode::NOT_FOUND, "FIT-002"),
            Self::PathFitNotFound(..) => (StatusCode::NOT_FOUND, "FIT-001"),
            Self::FitAddFailed(err) => match err {
                rs::err::AddFitError::FleetSetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-003"),
            },
            Self::FitChangeFailed(_, err_l1) => match err_l1 {
                // Fit
                rs::err::ChangeFitEnumError::FitChangeFailed(rs::err::FitChangeFitError::FleetSetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-004")
                }
                // Item
                rs::err::ChangeFitEnumError::ItemRemoveFailed(err_l2) => match err_l2 {
                    rs::err::GetItemRemoveItemError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemRemoveItemError::ItemRemoveFailed(
                        rs::err::core::RemoveItemError::UnremovableAutocharge,
                    ) => (StatusCode::BAD_REQUEST, "ACH-002"),
                },
                // Item - autocharge
                rs::err::ChangeFitEnumError::AutochargeChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeAutochargeError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeAutochargeError::ItemIsNotAutocharge(..) => {
                        (StatusCode::BAD_REQUEST, "ACH-001")
                    }
                },
                // Item - booster
                rs::err::ChangeFitEnumError::BoosterChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeBoosterError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeBoosterError::ItemIsNotBooster(..) => (StatusCode::BAD_REQUEST, "BST-001"),
                },
                // Item - character
                rs::err::ChangeFitEnumError::CharacterChangeFailed(
                    rs::err::FitChangeCharacterError::FitNoCharacter(..),
                ) => (StatusCode::BAD_REQUEST, "CHR-002"),
                // Item - charge
                rs::err::ChangeFitEnumError::ChargeChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeChargeError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeChargeError::ItemIsNotCharge(..) => (StatusCode::BAD_REQUEST, "CHG-001"),
                },
                // Item - drone
                rs::err::ChangeFitEnumError::DroneAddFailed(rs::err::FitAddDroneError::ProjAddFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "DRN-002")
                }
                rs::err::ChangeFitEnumError::DroneChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeDroneError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeDroneError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::GetItemChangeDroneError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::GetItemChangeDroneError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::GetItemChangeDroneError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                // Item - fighter
                rs::err::ChangeFitEnumError::FighterAddFailed(rs::err::FitAddFighterError::ProjAddFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FTR-002")
                }
                rs::err::ChangeFitEnumError::FighterChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeFighterError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeFighterError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::GetItemChangeFighterError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::GetItemChangeFighterError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                // Item - fit-wide effect
                rs::err::ChangeFitEnumError::FwEffectChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeFwEffectError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeFwEffectError::ItemIsNotFwEffect(..) => (StatusCode::BAD_REQUEST, "FWE-001"),
                },
                // Item - implant
                rs::err::ChangeFitEnumError::ImplantChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeImplantError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeImplantError::ItemIsNotImplant(..) => (StatusCode::BAD_REQUEST, "IMP-001"),
                },
                // Item - module
                rs::err::ChangeFitEnumError::ModuleAddFailed(rs::err::FitAddModuleError::ProjAddFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "MOD-002")
                }
                rs::err::ChangeFitEnumError::ModuleChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeModuleError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeModuleError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::GetItemChangeModuleError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::GetItemChangeModuleError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::GetItemChangeModuleError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                // Item - rig
                rs::err::ChangeFitEnumError::RigChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeRigError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeRigError::ItemIsNotRig(..) => (StatusCode::BAD_REQUEST, "RIG-001"),
                },
                // Item - service
                rs::err::ChangeFitEnumError::ServiceChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeServiceError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeServiceError::ItemIsNotService(..) => (StatusCode::BAD_REQUEST, "SVC-001"),
                },
                // Item - ship
                rs::err::ChangeFitEnumError::ShipChangeFailed(rs::err::FitChangeShipError::FitNoShip(..)) => {
                    (StatusCode::BAD_REQUEST, "SHP-002")
                }
                // Item - skill
                rs::err::ChangeFitEnumError::SkillAddFailed(rs::err::FitAddSkillError::SkillAddFailed(
                    rs::err::core::AddSkillError::SkillIdCollision(..),
                )) => (StatusCode::BAD_REQUEST, "SKL-002"),
                rs::err::ChangeFitEnumError::SkillChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeSkillError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeSkillError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::GetItemChangeSkillError::TypeIdSetFailed(
                        rs::err::core::SetSkillTypeIdError::SkillIdCollision(..),
                    ) => (StatusCode::BAD_REQUEST, "SKL-003"),
                },
                // Item - stance
                rs::err::ChangeFitEnumError::StanceChangeFailed(rs::err::FitChangeStanceError::FitNoStance(..)) => {
                    (StatusCode::BAD_REQUEST, "STC-002")
                }
                // Item - subsystem
                rs::err::ChangeFitEnumError::SubsystemChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeSubsystemError::ItemGetFailed(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeSubsystemError::ItemIsNotSubsystem(..) => {
                        (StatusCode::BAD_REQUEST, "SUB-001")
                    }
                },
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Item-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathItemParseFailed(..) => (StatusCode::NOT_FOUND, "ITM-002"),
            Self::PathItemNotFound(..) => (StatusCode::NOT_FOUND, "ITM-001"),
            Self::ItemAddFailed(err_l1) => match err_l1 {
                rs::err::AddItemEnumError::BoosterFailed(rs::err::GetFitAddBoosterError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::AddItemEnumError::CharacterFailed(rs::err::GetFitSetCharacterError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::AddItemEnumError::DroneFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddDroneError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddDroneError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "DRN-002"),
                },
                rs::err::AddItemEnumError::FighterFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddFighterError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddFighterError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "FTR-002"),
                },
                rs::err::AddItemEnumError::FwEffectFailed(rs::err::GetFitAddFwEffectError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::AddItemEnumError::ImplantFailed(rs::err::GetFitAddImplantError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::AddItemEnumError::ModuleFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddModuleError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddModuleError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "MOD-002"),
                },
                rs::err::AddItemEnumError::ProjEffectFailed(rs::err::AddProjEffectError::ProjAddFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "PJE-002")
                }
                rs::err::AddItemEnumError::RigFailed(rs::err::GetFitAddRigError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::AddItemEnumError::ServiceFailed(rs::err::GetFitAddServiceError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::AddItemEnumError::ShipFailed(rs::err::GetFitSetShipError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::AddItemEnumError::SkillFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddSkillError::FitGetFailed(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddSkillError::SkillAddFailed(rs::err::core::AddSkillError::SkillIdCollision(
                        ..,
                    )) => (StatusCode::BAD_REQUEST, "SKL-002"),
                },
                rs::err::AddItemEnumError::StanceFailed(rs::err::GetFitSetStanceError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::AddItemEnumError::SubsystemFailed(rs::err::GetFitAddSubsystemError::FitGetFailed(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
            },
            Self::ItemChangeFailed(err_l1) => match err_l1 {
                rs::err::ChangeItemEnumError::AutochargeFailed(
                    rs::err::ItemChangeAutochargeError::ItemIsNotAutocharge(..),
                ) => (StatusCode::BAD_REQUEST, "ACH-001"),
                rs::err::ChangeItemEnumError::BoosterFailed(rs::err::ItemChangeBoosterError::ItemIsNotBooster(..)) => {
                    (StatusCode::BAD_REQUEST, "BST-001")
                }
                rs::err::ChangeItemEnumError::CharacterFailed(
                    rs::err::ItemChangeCharacterError::ItemIsNotCharacter(..),
                ) => (StatusCode::BAD_REQUEST, "CHR-001"),
                rs::err::ChangeItemEnumError::ChargeFailed(rs::err::ItemChangeChargeError::ItemIsNotCharge(..)) => {
                    (StatusCode::BAD_REQUEST, "CHG-001")
                }
                rs::err::ChangeItemEnumError::DroneFailed(err_l2) => match err_l2 {
                    rs::err::ItemChangeDroneError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::ItemChangeDroneError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::ItemChangeDroneError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::ItemChangeDroneError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                rs::err::ChangeItemEnumError::FighterFailed(err_l2) => match err_l2 {
                    rs::err::ItemChangeFighterError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::ItemChangeFighterError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::ItemChangeFighterError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                rs::err::ChangeItemEnumError::FwEffectFailed(rs::err::ItemChangeFwEffectError::ItemIsNotFwEffect(
                    _,
                )) => (StatusCode::BAD_REQUEST, "FWE-001"),
                rs::err::ChangeItemEnumError::ImplantFailed(rs::err::ItemChangeImplantError::ItemIsNotImplant(..)) => {
                    (StatusCode::BAD_REQUEST, "IMP-001")
                }
                rs::err::ChangeItemEnumError::ModuleFailed(err_l2) => match err_l2 {
                    rs::err::ItemChangeModuleError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::ItemChangeModuleError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::ItemChangeModuleError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::ItemChangeModuleError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                rs::err::ChangeItemEnumError::ProjEffectFailed(err_l2) => match err_l2 {
                    rs::err::ItemChangeProjEffectError::ItemIsNotProjEffect(..) => (StatusCode::BAD_REQUEST, "PJE-001"),
                    rs::err::ItemChangeProjEffectError::ProjAddFailed(..) => (StatusCode::BAD_REQUEST, "PJE-003"),
                    rs::err::ItemChangeProjEffectError::ProjRemoveFailed(..) => (StatusCode::BAD_REQUEST, "PJE-004"),
                },
                rs::err::ChangeItemEnumError::RigFailed(rs::err::ItemChangeRigError::ItemIsNotRig(..)) => {
                    (StatusCode::BAD_REQUEST, "ITM-001")
                }
                rs::err::ChangeItemEnumError::ServiceFailed(rs::err::ItemChangeServiceError::ItemIsNotService(..)) => {
                    (StatusCode::BAD_REQUEST, "SVC-001")
                }
                rs::err::ChangeItemEnumError::ShipFailed(rs::err::ItemChangeShipError::ItemIsNotShip(..)) => {
                    (StatusCode::BAD_REQUEST, "SHP-001")
                }
                rs::err::ChangeItemEnumError::SkillFailed(err_l2) => match err_l2 {
                    rs::err::ItemChangeSkillError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::ItemChangeSkillError::TypeIdSetFailed(
                        rs::err::core::SetSkillTypeIdError::SkillIdCollision(..),
                    ) => (StatusCode::BAD_REQUEST, "SKL-003"),
                },
                rs::err::ChangeItemEnumError::StanceFailed(rs::err::ItemChangeStanceError::ItemIsNotStance(..)) => {
                    (StatusCode::BAD_REQUEST, "STC-001")
                }
                rs::err::ChangeItemEnumError::SubsystemFailed(
                    rs::err::ItemChangeSubsystemError::ItemIsNotSubsystem(..),
                ) => (StatusCode::BAD_REQUEST, "ITM-001"),
                rs::err::ChangeItemEnumError::SwEffectFailed(rs::err::ItemChangeSwEffectError::ItemIsNotSwEffect(
                    _,
                )) => (StatusCode::BAD_REQUEST, "SWE-001"),
            },
            Self::ItemRemoveFailed(rs::err::RemoveItemError(rs::err::ItemRemoveItemError::ItemRemoveFailed(
                rs::err::core::RemoveItemError::UnremovableAutocharge,
            ))) => (StatusCode::FORBIDDEN, "ACH-002"),
        }
    }
    fn get_cmd_index(&self) -> Option<usize> {
        match self {
            Self::BatchParseFailed(index, _) => Some(*index),
            Self::BackrefRenderFailed(index, _) => Some(*index),
            Self::SolChangeFailed(index, _) => Some(*index),
            Self::FitChangeFailed(index, _) => Some(*index),
            _ => None,
        }
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
            rs::err::ChangeSolError::RenderFailed(index, inner) => Self::BackrefRenderFailed(index, inner),
            rs::err::ChangeSolError::ExecFailed(index, inner) => Self::SolChangeFailed(index, inner),
        }
    }
}
impl From<rs::err::ChangeFitError> for ApiError {
    fn from(err: rs::err::ChangeFitError) -> Self {
        match err {
            rs::err::ChangeFitError::RenderFailed(index, inner) => Self::BackrefRenderFailed(index, inner),
            rs::err::ChangeFitError::ExecFailed(index, inner) => Self::FitChangeFailed(index, inner),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (http_code, api_code) = self.get_codes();
        let cmd_index = self.get_cmd_index();
        let payload = ApiErrorResponse {
            code: api_code,
            message: self.to_string(),
            cmd_index,
        };
        (http_code, Json(payload)).into_response()
    }
}
