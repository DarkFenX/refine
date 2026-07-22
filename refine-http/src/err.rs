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
    // Source-related
    #[error("{0}")]
    PathSrcNotFound(#[from] rs::src::err::GetSrcError),
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
    #[error("{0}")]
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
    #[error("{0}")]
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
            Self::Query(_) => (StatusCode::BAD_REQUEST, "PRM-001"),
            Self::Json(_) => (StatusCode::BAD_REQUEST, "JSN-001"),
            Self::BatchParseFailed(_, _) => (StatusCode::BAD_REQUEST, "JSN-002"),
            Self::BackrefRenderFailed(_, _) => (StatusCode::BAD_REQUEST, "BRF-001"),
            ////////////////////////////////////////////////////////////////////////////////////////
            // Source-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSrcNotFound(err) => match err {
                rs::src::err::GetSrcError::SrcNotFound(_) => (StatusCode::NOT_FOUND, "SRC-001"),
                rs::src::err::GetSrcError::DefaultNotDefined => (StatusCode::NOT_FOUND, "SRC-002"),
            },
            Self::SrcAddFailed(err) => match err {
                rs::src::err::AddSrcError::SrcAliasNotAvailable(_) => (StatusCode::FORBIDDEN, "SRC-003"),
                rs::src::err::AddSrcError::EdhInitFailed(_) => (StatusCode::BAD_REQUEST, "EDH-001"),
                rs::src::err::AddSrcError::SrcInitFailed(_) => (StatusCode::UNPROCESSABLE_ENTITY, "SNT-001"),
            },
            Self::SrcRemoveFailed(err) => match err {
                rs::src::err::RemoveSrcError::SrcNotFound(_) => (StatusCode::NOT_FOUND, "SRC-004"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Solar system-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathSolParseFailed(_) => (StatusCode::NOT_FOUND, "SOL-002"),
            Self::PathSolNotFound(err) => match err {
                rs::err::GetSolError::SolNotFound(_) => (StatusCode::NOT_FOUND, "SOL-001"),
            },
            Self::SolAddFailed(err) => match err {
                rs::err::AddSolError::GetSrcFailed(_) => (StatusCode::BAD_REQUEST, "SOL-003"),
            },
            Self::SolChangeFailed(_, err_l1) => match err_l1 {
                // Fleets
                rs::err::ChangeSolEnumError::FleetAddFailed(rs::err::AddFleetError::FitAddFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FLT-003")
                }
                rs::err::ChangeSolEnumError::FleetChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetFleetChangeFleetError::FleetGetFailed(_) => (StatusCode::BAD_REQUEST, "FLT-001"),
                    rs::err::GetFleetChangeFleetError::FitAddFailed(_) => (StatusCode::BAD_REQUEST, "FLT-004"),
                    rs::err::GetFleetChangeFleetError::FitRemoveFailed(_) => (StatusCode::BAD_REQUEST, "FLT-005"),
                },
                rs::err::ChangeSolEnumError::FleetRemoveFailed(rs::err::GetFleetRemoveFleetError::FleetGetFailed(
                    _,
                )) => (StatusCode::BAD_REQUEST, "FLT-001"),
                // Fits
                rs::err::ChangeSolEnumError::FitAddFailed(rs::err::AddFitError::FleetSetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-003")
                }
                rs::err::ChangeSolEnumError::FitChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetFitChangeFitError::FitGetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitChangeFitError::FleetSetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-004"),
                },
                rs::err::ChangeSolEnumError::FitRemoveFailed(rs::err::GetFitRemoveFitError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item
                rs::err::ChangeSolEnumError::ItemRemoveFailed(err_l2) => match err_l2 {
                    rs::err::GetItemRemoveItemError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemRemoveItemError::ItemRemoveFailed(
                        rs::err::core::RemoveItemError::UnremovableAutocharge,
                    ) => (StatusCode::BAD_REQUEST, "ACH-002"),
                },
                // Item - autocharge
                rs::err::ChangeSolEnumError::AutochargeChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeAutochargeError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeAutochargeError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "ACH-001"),
                },
                // Item - booster
                rs::err::ChangeSolEnumError::BoosterAddFailed(rs::err::GetFitAddBoosterError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::BoosterChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeBoosterError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeBoosterError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "BST-001"),
                },
                // Item - character
                rs::err::ChangeSolEnumError::CharacterSetFailed(rs::err::GetFitSetCharacterError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::CharacterChangeFailed(err_l2) => match err_l2 {
                    rs::err::ChangeCharacterError::CharacterChangeViaFitFailed(err_l3) => match err_l3 {
                        rs::err::GetFitChangeCharacterError::FitGetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::GetFitChangeCharacterError::FitNoCharacter(_) => (StatusCode::BAD_REQUEST, "CHR-001"),
                    },
                    rs::err::ChangeCharacterError::CharacterChangeViaItemFailed(
                        rs::err::GetItemChangeCharacterError::ItemGetFailed(_),
                    ) => (StatusCode::BAD_REQUEST, "ITM-001"),
                },
                rs::err::ChangeSolEnumError::CharacterUnsetFailed(
                    rs::err::GetFitUnsetCharacterError::FitGetFailed(_),
                ) => (StatusCode::BAD_REQUEST, "FIT-001"),
                // Item - charge
                rs::err::ChangeSolEnumError::ChargeChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeChargeError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeChargeError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "CHG-001"),
                },
                // Item - drone
                rs::err::ChangeSolEnumError::DroneAddFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddDroneError::FitGetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddDroneError::ProjAddFailed(_) => (StatusCode::BAD_REQUEST, "DRN-002"),
                },
                rs::err::ChangeSolEnumError::DroneChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeDroneError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeDroneError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::err::GetItemChangeDroneError::NotMutated(_) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::err::GetItemChangeDroneError::ProjAddFailed(_) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::err::GetItemChangeDroneError::ProjRemoveFailed(_) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                // Item - fighter
                rs::err::ChangeSolEnumError::FighterAddFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddFighterError::FitGetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddFighterError::ProjAddFailed(_) => (StatusCode::BAD_REQUEST, "FTR-002"),
                },
                rs::err::ChangeSolEnumError::FighterChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeFighterError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeFighterError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::err::GetItemChangeFighterError::ProjAddFailed(_) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::err::GetItemChangeFighterError::ProjRemoveFailed(_) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                // Item - fit-wide effect
                rs::err::ChangeSolEnumError::FwEffectAddFailed(rs::err::GetFitAddFwEffectError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::FwEffectChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeFwEffectError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeFwEffectError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "FWE-001"),
                },
                // Item - implant
                rs::err::ChangeSolEnumError::ImplantAddFailed(rs::err::GetFitAddImplantError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::ImplantChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeImplantError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeImplantError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "IMP-001"),
                },
                // Item - module
                rs::err::ChangeSolEnumError::ModuleAddFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddModuleError::FitGetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddModuleError::ProjAddFailed(_) => (StatusCode::BAD_REQUEST, "MOD-002"),
                },
                rs::err::ChangeSolEnumError::ModuleChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeModuleError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeModuleError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::err::GetItemChangeModuleError::NotMutated(_) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::err::GetItemChangeModuleError::ProjAddFailed(_) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::err::GetItemChangeModuleError::ProjRemoveFailed(_) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                // Item - projected effect
                rs::err::ChangeSolEnumError::ProjEffectAddFailed(rs::err::AddProjEffectError::ProjAddFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "PJE-002")
                }
                rs::err::ChangeSolEnumError::ProjEffectChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeProjEffectError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeProjEffectError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "PJE-001"),
                    rs::err::GetItemChangeProjEffectError::ProjAddFailed(_) => (StatusCode::BAD_REQUEST, "PJE-003"),
                    rs::err::GetItemChangeProjEffectError::ProjRemoveFailed(_) => (StatusCode::BAD_REQUEST, "PJE-004"),
                },
                // Item - rig
                rs::err::ChangeSolEnumError::RigAddFailed(rs::err::GetFitAddRigError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::RigChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeRigError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeRigError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "RIG-001"),
                },
                // Item - service
                rs::err::ChangeSolEnumError::ServiceAddFailed(rs::err::GetFitAddServiceError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::ServiceChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeServiceError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeServiceError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "SVC-001"),
                },
                // Item - ship
                rs::err::ChangeSolEnumError::ShipSetFailed(rs::err::GetFitSetShipError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::ShipChangeFailed(err_l2) => match err_l2 {
                    rs::err::ChangeShipError::ShipChangeViaFitFailed(err_l3) => match err_l3 {
                        rs::err::GetFitChangeShipError::FitGetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::GetFitChangeShipError::FitNoShip(_) => (StatusCode::BAD_REQUEST, "SHP-001"),
                    },
                    rs::err::ChangeShipError::ShipChangeViaItemFailed(
                        rs::err::GetItemChangeShipError::ItemGetFailed(_),
                    ) => (StatusCode::BAD_REQUEST, "ITM-001"),
                },
                rs::err::ChangeSolEnumError::ShipUnsetFailed(rs::err::GetFitUnsetShipError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - skill
                rs::err::ChangeSolEnumError::SkillAddFailed(err_l2) => match err_l2 {
                    rs::err::GetFitAddSkillError::FitGetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-001"),
                    rs::err::GetFitAddSkillError::SkillAddFailed(rs::err::core::AddSkillError::SkillIdCollision(_)) => {
                        (StatusCode::BAD_REQUEST, "SKL-002")
                    }
                },
                rs::err::ChangeSolEnumError::SkillChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeSkillError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeSkillError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::err::GetItemChangeSkillError::TypeIdSetFailed(
                        rs::err::core::SetSkillTypeIdError::SkillIdCollision(_),
                    ) => (StatusCode::BAD_REQUEST, "SKL-003"),
                },
                // Item - stance
                rs::err::ChangeSolEnumError::StanceSetFailed(rs::err::GetFitSetStanceError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::StanceChangeFailed(err_l2) => match err_l2 {
                    rs::err::ChangeStanceError::StanceChangeViaFitFailed(err_l3) => match err_l3 {
                        rs::err::GetFitChangeStanceError::FitGetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-001"),
                        rs::err::GetFitChangeStanceError::FitNoStance(_) => (StatusCode::BAD_REQUEST, "STC-001"),
                    },
                    rs::err::ChangeStanceError::StanceChangeViaItemFailed(
                        rs::err::GetItemChangeStanceError::ItemGetFailed(_),
                    ) => (StatusCode::BAD_REQUEST, "ITM-001"),
                },
                rs::err::ChangeSolEnumError::StanceUnsetFailed(rs::err::GetFitUnsetStanceError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                // Item - subsystem
                rs::err::ChangeSolEnumError::SubsystemAddFailed(rs::err::GetFitAddSubsystemError::FitGetFailed(_)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::err::ChangeSolEnumError::SubsystemChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeSubsystemError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeSubsystemError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "SUB-001"),
                },
                // Item - system-wide effect
                rs::err::ChangeSolEnumError::SwEffectChangeFailed(err_l2) => match err_l2 {
                    rs::err::GetItemChangeSwEffectError::ItemGetFailed(_) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::GetItemChangeSwEffectError::ItemKindMismatch(_) => (StatusCode::BAD_REQUEST, "SWE-001"),
                },
            },
            Self::SolRemoveFailed(err) => match err {
                rs::err::RemoveSolError::SolNotFound(_) => (StatusCode::NOT_FOUND, "SOL-004"),
            },
            Self::SolSrcSwitch(err) => match err {
                rs::err::SolSwitchSrcError::SrcGetFailed(_) => (StatusCode::BAD_REQUEST, "SOL-005"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fleet-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFleetParseFailed(_) => (StatusCode::NOT_FOUND, "FLT-002"),
            Self::PathFleetNotFound(_) => (StatusCode::NOT_FOUND, "FLT-001"),
            Self::FleetAddFailed(err) => match err {
                rs::err::AddFleetError::FitAddFailed(_) => (StatusCode::BAD_REQUEST, "FLT-003"),
            },
            Self::FleetChangeFailed(rs::err::ChangeFleetError(err)) => match err {
                rs::err::FleetChangeFleetError::FitAddFailed(_) => (StatusCode::BAD_REQUEST, "FLT-004"),
                rs::err::FleetChangeFleetError::FitRemoveFailed(_) => (StatusCode::BAD_REQUEST, "FLT-005"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fit-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFitParseFailed(_) => (StatusCode::NOT_FOUND, "FIT-002"),
            Self::PathFitNotFound(_) => (StatusCode::NOT_FOUND, "FIT-001"),
            Self::FitAddFailed(err) => match err {
                rs::err::AddFitError::FleetSetFailed(_) => (StatusCode::BAD_REQUEST, "FIT-003"),
            },
            // TODO: adjust error codes based on specific responses
            Self::FitChangeFailed(_, _) => (StatusCode::BAD_REQUEST, "FIT-000"),
            ////////////////////////////////////////////////////////////////////////////////////////
            // Item-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathItemParseFailed(_) => (StatusCode::NOT_FOUND, "ITM-002"),
            Self::PathItemNotFound(_) => (StatusCode::NOT_FOUND, "ITM-001"),
            // TODO: adjust error codes based on specific responses
            Self::ItemAddFailed(_) => (StatusCode::BAD_REQUEST, "ITM-003"),
            // TODO: adjust error codes based on specific responses
            Self::ItemChangeFailed(_) => (StatusCode::BAD_REQUEST, "ITM-004"),
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
