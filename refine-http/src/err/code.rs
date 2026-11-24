use axum::{extract::rejection::JsonRejection, http::StatusCode};

use crate::err::ApiError;

impl ApiError {
    pub(super) fn get_codes(&self) -> (StatusCode, &'static str) {
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
            Self::SolChange(err) => sol_change_enum(err),
            Self::SolBatchCtl(err) => sol_change_enum(&err.error),
            Self::SolBatchInfo(err) => match &err.error {
                rs::info::err::SolInfoEnumError::Fleet(rs::info::err::FleetGetFleetInfoError::FleetGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FLT-001")
                }
                rs::info::err::SolInfoEnumError::Fit(rs::info::err::FitGetFitInfoError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::info::err::SolInfoEnumError::Item(rs::info::err::ItemGetItemInfoError::ItemGet(..)) => {
                    (StatusCode::BAD_REQUEST, "ITM-001")
                }
            },
            Self::SolBatchStats(err) => match &err.error {
                rs::stats::err::SolStatsEnumError::Fleet(rs::stats::err::FleetGetFleetStatsError::FleetGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FLT-001")
                }
                rs::stats::err::SolStatsEnumError::Fit(rs::stats::err::FitGetFitStatsError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::stats::err::SolStatsEnumError::Item(rs::stats::err::ItemGetItemStatsError::ItemGet(..)) => {
                    (StatusCode::BAD_REQUEST, "ITM-001")
                }
            },
            Self::SolBatchVal(err) => match &err.error {
                rs::val::err::SolValEnumError::Fit(rs::val::err::FitGetFitValError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
            },
            Self::SolBatchTryItems(err) => match &err.error {
                rs::trial::err::SolTryItemsEnumError::Fit(rs::trial::err::FitGetFitTryItemsError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
            },
            Self::SolRemove(err) => match err {
                rs::err::SolRemoveError::SolNotFound(..) => (StatusCode::NOT_FOUND, "SOL-001"),
            },
            Self::SolSrcSwitch(err) => match err {
                rs::err::SolSwitchSrcError::SrcGet(..) => (StatusCode::BAD_REQUEST, "SOL-003"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fleet-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFleetParse(..) => (StatusCode::NOT_FOUND, "FLT-002"),
            Self::PathFleetNotFound(..) => (StatusCode::NOT_FOUND, "FLT-001"),
            Self::FleetAdd(err) => match err {
                rs::ctl::err::FleetAddError::FitAdd(..) => (StatusCode::BAD_REQUEST, "FLT-003"),
            },
            Self::FleetChange(err) => match err {
                rs::ctl::err::FleetChangeError::FitAdd(..) => (StatusCode::BAD_REQUEST, "FLT-004"),
                rs::ctl::err::FleetChangeError::FitRemove(..) => (StatusCode::BAD_REQUEST, "FLT-005"),
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Fit-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathFitParse(..) => (StatusCode::NOT_FOUND, "FIT-002"),
            Self::PathFitNotFound(..) => (StatusCode::NOT_FOUND, "FIT-001"),
            Self::FitAdd(err) => match err {
                rs::ctl::err::FitAddError::FleetSet(..) => (StatusCode::BAD_REQUEST, "FIT-003"),
            },
            Self::FitChange(err) => fit_change_enum(err),
            Self::FitBatchCtl(err) => fit_change_enum(&err.error),
            Self::FitBatchInfo(err) => match &err.error {
                rs::info::err::FitInfoEnumError::Item(rs::info::err::FitItemGetItemInfoError::ItemGet(err_l3)) => {
                    fit_item_get(err_l3)
                }
            },
            Self::FitBatchStats(err) => match &err.error {
                rs::stats::err::FitStatsEnumError::Item(rs::stats::err::FitItemGetItemStatsError::ItemGet(err_l3)) => {
                    fit_item_get(err_l3)
                }
            },
            ////////////////////////////////////////////////////////////////////////////////////////
            // Item-related
            ////////////////////////////////////////////////////////////////////////////////////////
            Self::PathItemParse(..) => (StatusCode::NOT_FOUND, "ITM-002"),
            Self::PathItemNotFound(..) => (StatusCode::NOT_FOUND, "ITM-001"),
            Self::ItemAdd(err_l1) => match err_l1 {
                rs::ctl::err::ItemAddEnumError::Autodetect(err_l2) => fit_get_item_autodetect_add(err_l2),
                rs::ctl::err::ItemAddEnumError::Booster(rs::ctl::err::FitGetBoosterAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::ctl::err::ItemAddEnumError::Character(rs::ctl::err::FitGetCharacterSetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::ctl::err::ItemAddEnumError::Drone(err_l2) => fit_get_drone_add(err_l2),
                rs::ctl::err::ItemAddEnumError::Fighter(err_l2) => fit_get_fighter_add(err_l2),
                rs::ctl::err::ItemAddEnumError::FwEffect(rs::ctl::err::FitGetFwEffectAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::ctl::err::ItemAddEnumError::Implant(rs::ctl::err::FitGetImplantAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::ctl::err::ItemAddEnumError::Module(err_l2) => fit_get_module_add(err_l2),
                rs::ctl::err::ItemAddEnumError::ProjEffect(rs::ctl::err::ProjEffectAddError::ProjAdd(..)) => {
                    (StatusCode::BAD_REQUEST, "PJE-002")
                }
                rs::ctl::err::ItemAddEnumError::Rig(rs::ctl::err::FitGetRigAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::ctl::err::ItemAddEnumError::Service(rs::ctl::err::FitGetServiceAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::ctl::err::ItemAddEnumError::Ship(rs::ctl::err::FitGetShipSetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::ctl::err::ItemAddEnumError::Skill(err_l2) => fit_get_skill_add(err_l2),
                rs::ctl::err::ItemAddEnumError::Stance(rs::ctl::err::FitGetStanceSetError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
                rs::ctl::err::ItemAddEnumError::Subsystem(rs::ctl::err::FitGetSubsystemAddError::FitGet(..)) => {
                    (StatusCode::BAD_REQUEST, "FIT-001")
                }
            },
            Self::ItemChange(err_l1) => match err_l1 {
                rs::ctl::err::ItemChangeEnumError::Autocharge(
                    rs::ctl::err::AutochargeChangeError::ItemIsNotAutocharge(..),
                ) => (StatusCode::BAD_REQUEST, "ACH-001"),
                rs::ctl::err::ItemChangeEnumError::Booster(rs::ctl::err::BoosterChangeError::ItemIsNotBooster(..)) => {
                    (StatusCode::BAD_REQUEST, "BST-001")
                }
                rs::ctl::err::ItemChangeEnumError::Character(
                    rs::ctl::err::ItemCharacterChangeError::ItemIsNotCharacter(..),
                ) => (StatusCode::BAD_REQUEST, "CHR-001"),
                rs::ctl::err::ItemChangeEnumError::Charge(rs::ctl::err::ChargeChangeError::ItemIsNotCharge(..)) => {
                    (StatusCode::BAD_REQUEST, "CHG-001")
                }
                rs::ctl::err::ItemChangeEnumError::Drone(err_l2) => match err_l2 {
                    rs::ctl::err::DroneChangeError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
                    rs::ctl::err::DroneChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
                    rs::ctl::err::DroneChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
                    rs::ctl::err::DroneChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
                },
                rs::ctl::err::ItemChangeEnumError::Fighter(err_l2) => match err_l2 {
                    rs::ctl::err::FighterChangeError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
                    rs::ctl::err::FighterChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
                    rs::ctl::err::FighterChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
                },
                rs::ctl::err::ItemChangeEnumError::FwEffect(rs::ctl::err::FwEffectChangeError::ItemIsNotFwEffect(
                    ..,
                )) => (StatusCode::BAD_REQUEST, "FWE-001"),
                rs::ctl::err::ItemChangeEnumError::Implant(rs::ctl::err::ImplantChangeError::ItemIsNotImplant(..)) => {
                    (StatusCode::BAD_REQUEST, "IMP-001")
                }
                rs::ctl::err::ItemChangeEnumError::Module(err_l2) => match err_l2 {
                    rs::ctl::err::ModuleChangeError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
                    rs::ctl::err::ModuleChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
                    rs::ctl::err::ModuleChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
                    rs::ctl::err::ModuleChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
                },
                rs::ctl::err::ItemChangeEnumError::ProjEffect(err_l2) => match err_l2 {
                    rs::ctl::err::ProjEffectChangeError::ItemIsNotProjEffect(..) => {
                        (StatusCode::BAD_REQUEST, "PJE-001")
                    }
                    rs::ctl::err::ProjEffectChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "PJE-003"),
                    rs::ctl::err::ProjEffectChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "PJE-004"),
                },
                rs::ctl::err::ItemChangeEnumError::Rig(rs::ctl::err::RigChangeError::ItemIsNotRig(..)) => {
                    (StatusCode::BAD_REQUEST, "RIG-001")
                }
                rs::ctl::err::ItemChangeEnumError::Service(rs::ctl::err::ServiceChangeError::ItemIsNotService(..)) => {
                    (StatusCode::BAD_REQUEST, "SVC-001")
                }
                rs::ctl::err::ItemChangeEnumError::Ship(rs::ctl::err::ItemShipChangeError::ItemIsNotShip(..)) => {
                    (StatusCode::BAD_REQUEST, "SHP-001")
                }
                rs::ctl::err::ItemChangeEnumError::Skill(err_l2) => match err_l2 {
                    rs::ctl::err::SkillChangeError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
                    rs::ctl::err::SkillChangeError::TypeIdSet(
                        rs::err::core::SkillTypeIdSetError::SkillIdCollision(..),
                    ) => (StatusCode::BAD_REQUEST, "SKL-003"),
                },
                rs::ctl::err::ItemChangeEnumError::Stance(rs::ctl::err::ItemStanceChangeError::ItemIsNotStance(..)) => {
                    (StatusCode::BAD_REQUEST, "STC-001")
                }
                rs::ctl::err::ItemChangeEnumError::Subsystem(
                    rs::ctl::err::SubsystemChangeError::ItemIsNotSubsystem(..),
                ) => (StatusCode::BAD_REQUEST, "SUB-001"),
                rs::ctl::err::ItemChangeEnumError::SwEffect(rs::ctl::err::SwEffectChangeError::ItemIsNotSwEffect(
                    ..,
                )) => (StatusCode::BAD_REQUEST, "SWE-001"),
            },
            Self::ItemRemove(rs::ctl::err::ItemRemoveError::ItemRemove(
                rs::err::core::ItemRemoveError::UnremovableAutocharge,
            )) => (StatusCode::FORBIDDEN, "ACH-002"),
        }
    }
    pub(super) fn get_cmd_index(&self) -> Option<usize> {
        match self {
            Self::BatchParse(err) => Some(err.index),
            Self::BatchBackrefResolve(err) => Some(err.index),
            Self::SolBatchCtl(err) => Some(err.index),
            Self::SolBatchInfo(err) => Some(err.index),
            Self::SolBatchStats(err) => Some(err.index),
            Self::SolBatchVal(err) => Some(err.index),
            Self::SolBatchTryItems(err) => Some(err.index),
            Self::FitBatchCtl(err) => Some(err.index),
            Self::FitBatchInfo(err) => Some(err.index),
            Self::FitBatchStats(err) => Some(err.index),
            _ => None,
        }
    }
}

fn sol_change_enum(err: &rs::ctl::err::SolChangeEnumError) -> (StatusCode, &'static str) {
    match err {
        // Fleets
        rs::ctl::err::SolChangeEnumError::FleetAdd(rs::ctl::err::FleetAddError::FitAdd(..)) => {
            (StatusCode::BAD_REQUEST, "FLT-003")
        }
        rs::ctl::err::SolChangeEnumError::FleetChange(err_l2) => match err_l2 {
            rs::ctl::err::FleetGetFleetChangeError::FleetGet(..) => (StatusCode::BAD_REQUEST, "FLT-001"),
            rs::ctl::err::FleetGetFleetChangeError::FitAdd(..) => (StatusCode::BAD_REQUEST, "FLT-004"),
            rs::ctl::err::FleetGetFleetChangeError::FitRemove(..) => (StatusCode::BAD_REQUEST, "FLT-005"),
        },
        rs::ctl::err::SolChangeEnumError::FleetRemove(rs::ctl::err::FleetGetFleetRemoveError::FleetGet(..)) => {
            (StatusCode::BAD_REQUEST, "FLT-001")
        }
        // Fits
        rs::ctl::err::SolChangeEnumError::FitAdd(rs::ctl::err::FitAddError::FleetSet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-003")
        }
        rs::ctl::err::SolChangeEnumError::FitChange(err_l2) => match err_l2 {
            rs::ctl::err::FitGetFitChangeError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
            rs::ctl::err::FitGetFitChangeError::FleetSet(..) => (StatusCode::BAD_REQUEST, "FIT-004"),
        },
        rs::ctl::err::SolChangeEnumError::FitRemove(rs::ctl::err::FitGetFitRemoveError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        // Item
        rs::ctl::err::SolChangeEnumError::ItemAutodetectAdd(err_l2) => fit_get_item_autodetect_add(err_l2),
        rs::ctl::err::SolChangeEnumError::ItemRemove(err_l2) => gen_item_get_item_remove(err_l2, item_get),
        // Item - autocharge
        rs::ctl::err::SolChangeEnumError::AutochargeChange(err_l2) => gen_item_get_autocharge_change(err_l2, item_get),
        // Item - booster
        rs::ctl::err::SolChangeEnumError::BoosterAdd(rs::ctl::err::FitGetBoosterAddError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        rs::ctl::err::SolChangeEnumError::BoosterChange(err_l2) => gen_item_get_booster_change(err_l2, item_get),
        // Item - character
        rs::ctl::err::SolChangeEnumError::CharacterSet(rs::ctl::err::FitGetCharacterSetError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        rs::ctl::err::SolChangeEnumError::CharacterChange(err_l2) => match err_l2 {
            rs::ctl::err::CharacterChangeError::ViaFit(err_l3) => match err_l3 {
                rs::ctl::err::FitGetCharacterChangeError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                rs::ctl::err::FitGetCharacterChangeError::FitNoCharacter(..) => (StatusCode::BAD_REQUEST, "CHR-002"),
            },
            rs::ctl::err::CharacterChangeError::ViaItem(rs::ctl::err::ItemGetCharacterChangeError::ItemGet(err_l3)) => {
                match err_l3 {
                    rs::err::core::CharacterGetError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::core::CharacterGetError::ItemIsNotCharacter(..) => (StatusCode::BAD_REQUEST, "CHR-001"),
                }
            }
        },
        rs::ctl::err::SolChangeEnumError::CharacterUnset(rs::ctl::err::FitGetCharacterUnsetError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        // Item - charge
        rs::ctl::err::SolChangeEnumError::ChargeChange(err_l2) => gen_item_get_charge_change(err_l2, item_get),
        // Item - drone
        rs::ctl::err::SolChangeEnumError::DroneAdd(err_l2) => fit_get_drone_add(err_l2),
        rs::ctl::err::SolChangeEnumError::DroneChange(err_l2) => gen_item_get_drone_change(err_l2, item_get),
        // Item - fighter
        rs::ctl::err::SolChangeEnumError::FighterAdd(err_l2) => fit_get_fighter_add(err_l2),
        rs::ctl::err::SolChangeEnumError::FighterChange(err_l2) => gen_item_get_fighter_change(err_l2, item_get),
        // Item - fit-wide effect
        rs::ctl::err::SolChangeEnumError::FwEffectAdd(rs::ctl::err::FitGetFwEffectAddError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        rs::ctl::err::SolChangeEnumError::FwEffectChange(err_l2) => gen_item_get_fw_effect_change(err_l2, item_get),
        // Item - implant
        rs::ctl::err::SolChangeEnumError::ImplantAdd(rs::ctl::err::FitGetImplantAddError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        rs::ctl::err::SolChangeEnumError::ImplantChange(err_l2) => gen_item_get_implant_change(err_l2, item_get),
        // Item - module
        rs::ctl::err::SolChangeEnumError::ModuleAdd(err_l2) => fit_get_module_add(err_l2),
        rs::ctl::err::SolChangeEnumError::ModuleChange(err_l2) => gen_item_get_module_change(err_l2, item_get),
        // Item - projected effect
        rs::ctl::err::SolChangeEnumError::ProjEffectAdd(rs::ctl::err::ProjEffectAddError::ProjAdd(..)) => {
            (StatusCode::BAD_REQUEST, "PJE-002")
        }
        rs::ctl::err::SolChangeEnumError::ProjEffectChange(err_l2) => match err_l2 {
            rs::ctl::err::ItemGetProjEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
            rs::ctl::err::ItemGetProjEffectChangeError::ItemIsNotProjEffect(..) => (StatusCode::BAD_REQUEST, "PJE-001"),
            rs::ctl::err::ItemGetProjEffectChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "PJE-003"),
            rs::ctl::err::ItemGetProjEffectChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "PJE-004"),
        },
        // Item - rig
        rs::ctl::err::SolChangeEnumError::RigAdd(rs::ctl::err::FitGetRigAddError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        rs::ctl::err::SolChangeEnumError::RigChange(err_l2) => gen_item_get_rig_change(err_l2, item_get),
        // Item - service
        rs::ctl::err::SolChangeEnumError::ServiceAdd(rs::ctl::err::FitGetServiceAddError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        rs::ctl::err::SolChangeEnumError::ServiceChange(err_l2) => gen_item_get_service_change(err_l2, item_get),
        // Item - ship
        rs::ctl::err::SolChangeEnumError::ShipSet(rs::ctl::err::FitGetShipSetError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        rs::ctl::err::SolChangeEnumError::ShipChange(err_l2) => match err_l2 {
            rs::ctl::err::ShipChangeError::ViaFit(err_l3) => match err_l3 {
                rs::ctl::err::FitGetShipChangeError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                rs::ctl::err::FitGetShipChangeError::FitNoShip(..) => (StatusCode::BAD_REQUEST, "SHP-002"),
            },
            rs::ctl::err::ShipChangeError::ViaItem(rs::ctl::err::ItemGetShipChangeError::ItemGet(err_l3)) => {
                match err_l3 {
                    rs::err::core::ShipGetError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::core::ShipGetError::ItemIsNotShip(..) => (StatusCode::BAD_REQUEST, "SHP-001"),
                }
            }
        },
        rs::ctl::err::SolChangeEnumError::ShipUnset(rs::ctl::err::FitGetShipUnsetError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        // Item - skill
        rs::ctl::err::SolChangeEnumError::SkillAdd(err_l2) => fit_get_skill_add(err_l2),
        rs::ctl::err::SolChangeEnumError::SkillChange(err_l2) => gen_item_get_skill_change(err_l2, item_get),
        // Item - stance
        rs::ctl::err::SolChangeEnumError::StanceSet(rs::ctl::err::FitGetStanceSetError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        rs::ctl::err::SolChangeEnumError::StanceChange(err_l2) => match err_l2 {
            rs::ctl::err::StanceChangeError::ViaFit(err_l3) => match err_l3 {
                rs::ctl::err::FitGetStanceChangeError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
                rs::ctl::err::FitGetStanceChangeError::FitNoStance(..) => (StatusCode::BAD_REQUEST, "STC-002"),
            },
            rs::ctl::err::StanceChangeError::ViaItem(rs::ctl::err::ItemGetStanceChangeError::ItemGet(err_l3)) => {
                match err_l3 {
                    rs::err::core::StanceGetError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
                    rs::err::core::StanceGetError::ItemIsNotStance(..) => (StatusCode::BAD_REQUEST, "STC-001"),
                }
            }
        },
        rs::ctl::err::SolChangeEnumError::StanceUnset(rs::ctl::err::FitGetStanceUnsetError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        // Item - subsystem
        rs::ctl::err::SolChangeEnumError::SubsystemAdd(rs::ctl::err::FitGetSubsystemAddError::FitGet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-001")
        }
        rs::ctl::err::SolChangeEnumError::SubsystemChange(err_l2) => gen_item_get_subsystem_change(err_l2, item_get),
        // Item - system-wide effect
        rs::ctl::err::SolChangeEnumError::SwEffectChange(err_l2) => match err_l2 {
            rs::ctl::err::ItemGetSwEffectChangeError::ItemGet(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
            rs::ctl::err::ItemGetSwEffectChangeError::ItemIsNotSwEffect(..) => (StatusCode::BAD_REQUEST, "SWE-001"),
        },
    }
}

fn fit_change_enum(err: &rs::ctl::err::FitChangeEnumError) -> (StatusCode, &'static str) {
    match err {
        // Fit
        rs::ctl::err::FitChangeEnumError::FitChange(rs::ctl::err::FitChangeError::FleetSet(..)) => {
            (StatusCode::BAD_REQUEST, "FIT-004")
        }
        // Item
        rs::ctl::err::FitChangeEnumError::ItemAutodetectAdd(
            rs::ctl::err::ItemAutodetectAddError::ItemAutodetectAdd(err_l2),
        ) => fit_add_item_auto(err_l2),
        rs::ctl::err::FitChangeEnumError::ItemRemove(err_l2) => gen_item_get_item_remove(err_l2, fit_item_get),
        // Item - autocharge
        rs::ctl::err::FitChangeEnumError::AutochargeChange(err_l2) => {
            gen_item_get_autocharge_change(err_l2, fit_item_get)
        }
        // Item - booster
        rs::ctl::err::FitChangeEnumError::BoosterChange(err_l2) => gen_item_get_booster_change(err_l2, fit_item_get),
        // Item - character
        rs::ctl::err::FitChangeEnumError::CharacterChange(rs::ctl::err::FitCharacterChangeError::FitNoCharacter(
            ..,
        )) => (StatusCode::BAD_REQUEST, "CHR-002"),
        // Item - charge
        rs::ctl::err::FitChangeEnumError::ChargeChange(err_l2) => gen_item_get_charge_change(err_l2, fit_item_get),
        // Item - drone
        rs::ctl::err::FitChangeEnumError::DroneAdd(rs::ctl::err::DroneAddError::ProjAdd(..)) => {
            (StatusCode::BAD_REQUEST, "DRN-002")
        }
        rs::ctl::err::FitChangeEnumError::DroneChange(err_l2) => gen_item_get_drone_change(err_l2, fit_item_get),
        // Item - fighter
        rs::ctl::err::FitChangeEnumError::FighterAdd(rs::ctl::err::FighterAddError::ProjAdd(..)) => {
            (StatusCode::BAD_REQUEST, "FTR-002")
        }
        rs::ctl::err::FitChangeEnumError::FighterChange(err_l2) => gen_item_get_fighter_change(err_l2, fit_item_get),
        // Item - fit-wide effect
        rs::ctl::err::FitChangeEnumError::FwEffectChange(err_l2) => gen_item_get_fw_effect_change(err_l2, fit_item_get),
        // Item - implant
        rs::ctl::err::FitChangeEnumError::ImplantChange(err_l2) => gen_item_get_implant_change(err_l2, fit_item_get),
        // Item - module
        rs::ctl::err::FitChangeEnumError::ModuleAdd(rs::ctl::err::ModuleAddError::ModuleAdd(
            rs::err::core::ModuleAddError::RackPosLimit(..),
        )) => (StatusCode::BAD_REQUEST, "MOD-006"),
        rs::ctl::err::FitChangeEnumError::ModuleAdd(rs::ctl::err::ModuleAddError::ProjAdd(..)) => {
            (StatusCode::BAD_REQUEST, "MOD-002")
        }
        rs::ctl::err::FitChangeEnumError::ModuleChange(err_l2) => gen_item_get_module_change(err_l2, fit_item_get),
        // Item - rig
        rs::ctl::err::FitChangeEnumError::RigChange(err_l2) => gen_item_get_rig_change(err_l2, fit_item_get),
        // Item - service
        rs::ctl::err::FitChangeEnumError::ServiceChange(err_l2) => gen_item_get_service_change(err_l2, fit_item_get),
        // Item - ship
        rs::ctl::err::FitChangeEnumError::ShipChange(rs::ctl::err::FitShipChangeError::FitNoShip(..)) => {
            (StatusCode::BAD_REQUEST, "SHP-002")
        }
        // Item - skill
        rs::ctl::err::FitChangeEnumError::SkillAdd(rs::ctl::err::SkillAddError::SkillAdd(
            rs::err::core::SkillAddError::SkillIdCollision(..),
        )) => (StatusCode::BAD_REQUEST, "SKL-002"),
        rs::ctl::err::FitChangeEnumError::SkillChange(err_l2) => gen_item_get_skill_change(err_l2, fit_item_get),
        // Item - stance
        rs::ctl::err::FitChangeEnumError::StanceChange(rs::ctl::err::FitStanceChangeError::FitNoStance(..)) => {
            (StatusCode::BAD_REQUEST, "STC-002")
        }
        // Item - subsystem
        rs::ctl::err::FitChangeEnumError::SubsystemChange(err_l2) => {
            gen_item_get_subsystem_change(err_l2, fit_item_get)
        }
    }
}

fn item_get(err: &rs::err::core::ItemGetError) -> (StatusCode, &'static str) {
    match err {
        rs::err::core::ItemGetError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
    }
}
fn fit_item_get(err: &rs::err::core::FitItemGetError) -> (StatusCode, &'static str) {
    match err {
        rs::err::core::FitItemGetError::ItemNotFound(..) => (StatusCode::BAD_REQUEST, "ITM-001"),
        rs::err::core::FitItemGetError::ItemIsNotOnThisFit(..) => (StatusCode::BAD_REQUEST, "ITM-003"),
    }
}
fn gen_item_get_item_remove<E>(
    err: &rs::ctl::err::GenItemGetItemRemoveError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetItemRemoveError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetItemRemoveError::ItemRemove(rs::err::core::ItemRemoveError::UnremovableAutocharge) => {
            (StatusCode::BAD_REQUEST, "ACH-002")
        }
    }
}

fn fit_get_item_autodetect_add(err: &rs::ctl::err::FitGetItemAutodetectAddError) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::FitGetItemAutodetectAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
        rs::ctl::err::FitGetItemAutodetectAddError::ItemAutodetectAdd(err_l2) => fit_add_item_auto(err_l2),
    }
}
fn fit_add_item_auto(err: &rs::err::core::FitItemAutodetectAddError) -> (StatusCode, &'static str) {
    match err {
        rs::err::core::FitItemAutodetectAddError::TypeId(..) => (StatusCode::BAD_REQUEST, "ITM-004"),
        rs::err::core::FitItemAutodetectAddError::KindUnknown => (StatusCode::BAD_REQUEST, "ITM-005"),
        rs::err::core::FitItemAutodetectAddError::KindInvalid(..) => (StatusCode::BAD_REQUEST, "ITM-006"),
        rs::err::core::FitItemAutodetectAddError::RackPosLimit(..) => (StatusCode::BAD_REQUEST, "MOD-006"),
    }
}

fn gen_item_get_autocharge_change<E>(
    err: &rs::ctl::err::GenItemGetAutochargeChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetAutochargeChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetAutochargeChangeError::ItemIsNotAutocharge(..) => (StatusCode::BAD_REQUEST, "ACH-001"),
    }
}

fn gen_item_get_booster_change<E>(
    err: &rs::ctl::err::GenItemGetBoosterChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetBoosterChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetBoosterChangeError::ItemIsNotBooster(..) => (StatusCode::BAD_REQUEST, "BST-001"),
    }
}

fn gen_item_get_charge_change<E>(
    err: &rs::ctl::err::GenItemGetChargeChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetChargeChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetChargeChangeError::ItemIsNotCharge(..) => (StatusCode::BAD_REQUEST, "CHG-001"),
    }
}

fn fit_get_drone_add(err: &rs::ctl::err::FitGetDroneAddError) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::FitGetDroneAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
        rs::ctl::err::FitGetDroneAddError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-002"),
    }
}
fn gen_item_get_drone_change<E>(
    err: &rs::ctl::err::GenItemGetDroneChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetDroneChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetDroneChangeError::ItemIsNotDrone(..) => (StatusCode::BAD_REQUEST, "DRN-001"),
        rs::ctl::err::GenItemGetDroneChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "DRN-005"),
        rs::ctl::err::GenItemGetDroneChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "DRN-003"),
        rs::ctl::err::GenItemGetDroneChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "DRN-004"),
    }
}

fn fit_get_fighter_add(err: &rs::ctl::err::FitGetFighterAddError) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::FitGetFighterAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
        rs::ctl::err::FitGetFighterAddError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-002"),
    }
}
fn gen_item_get_fighter_change<E>(
    err: &rs::ctl::err::GenItemGetFighterChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetFighterChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetFighterChangeError::ItemIsNotFighter(..) => (StatusCode::BAD_REQUEST, "FTR-001"),
        rs::ctl::err::GenItemGetFighterChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "FTR-003"),
        rs::ctl::err::GenItemGetFighterChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "FTR-004"),
    }
}

fn gen_item_get_fw_effect_change<E>(
    err: &rs::ctl::err::GenItemGetFwEffectChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetFwEffectChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetFwEffectChangeError::ItemIsNotFwEffect(..) => (StatusCode::BAD_REQUEST, "FWE-001"),
    }
}

fn gen_item_get_implant_change<E>(
    err: &rs::ctl::err::GenItemGetImplantChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetImplantChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetImplantChangeError::ItemIsNotImplant(..) => (StatusCode::BAD_REQUEST, "IMP-001"),
    }
}

fn fit_get_module_add(err: &rs::ctl::err::FitGetModuleAddError) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::FitGetModuleAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
        rs::ctl::err::FitGetModuleAddError::ModuleAdd(rs::err::core::ModuleAddError::RackPosLimit(..)) => {
            (StatusCode::BAD_REQUEST, "MOD-006")
        }
        rs::ctl::err::FitGetModuleAddError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-002"),
    }
}
fn gen_item_get_module_change<E>(
    err: &rs::ctl::err::GenItemGetModuleChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetModuleChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetModuleChangeError::ItemIsNotModule(..) => (StatusCode::BAD_REQUEST, "MOD-001"),
        rs::ctl::err::GenItemGetModuleChangeError::NotMutated(..) => (StatusCode::BAD_REQUEST, "MOD-005"),
        rs::ctl::err::GenItemGetModuleChangeError::ProjAdd(..) => (StatusCode::BAD_REQUEST, "MOD-003"),
        rs::ctl::err::GenItemGetModuleChangeError::ProjRemove(..) => (StatusCode::BAD_REQUEST, "MOD-004"),
    }
}

fn gen_item_get_rig_change<E>(
    err: &rs::ctl::err::GenItemGetRigChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetRigChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetRigChangeError::ItemIsNotRig(..) => (StatusCode::BAD_REQUEST, "RIG-001"),
    }
}

fn gen_item_get_service_change<E>(
    err: &rs::ctl::err::GenItemGetServiceChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetServiceChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetServiceChangeError::ItemIsNotService(..) => (StatusCode::BAD_REQUEST, "SVC-001"),
    }
}

fn fit_get_skill_add(err: &rs::ctl::err::FitGetSkillAddError) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::FitGetSkillAddError::FitGet(..) => (StatusCode::BAD_REQUEST, "FIT-001"),
        rs::ctl::err::FitGetSkillAddError::SkillAdd(rs::err::core::SkillAddError::SkillIdCollision(..)) => {
            (StatusCode::BAD_REQUEST, "SKL-002")
        }
    }
}
fn gen_item_get_skill_change<E>(
    err: &rs::ctl::err::GenItemGetSkillChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetSkillChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetSkillChangeError::ItemIsNotSkill(..) => (StatusCode::BAD_REQUEST, "SKL-001"),
        rs::ctl::err::GenItemGetSkillChangeError::TypeIdSet(rs::err::core::SkillTypeIdSetError::SkillIdCollision(
            ..,
        )) => (StatusCode::BAD_REQUEST, "SKL-003"),
    }
}

fn gen_item_get_subsystem_change<E>(
    err: &rs::ctl::err::GenItemGetSubsystemChangeError<E>,
    map_item_get: fn(&E) -> (StatusCode, &'static str),
) -> (StatusCode, &'static str) {
    match err {
        rs::ctl::err::GenItemGetSubsystemChangeError::ItemGet(err_l2) => map_item_get(err_l2),
        rs::ctl::err::GenItemGetSubsystemChangeError::ItemIsNotSubsystem(..) => (StatusCode::BAD_REQUEST, "SUB-001"),
    }
}
