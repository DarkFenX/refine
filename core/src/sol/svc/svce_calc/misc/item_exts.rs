use crate::{
    defs::SolItemId,
    sol::{
        item::{SolItem, SolShipKind},
        svc::svce_calc::SolLocationKind,
    },
};

impl SolItem {
    pub(in crate::sol::svc::svce_calc) fn get_root_loc_kind(&self) -> Option<SolLocationKind> {
        match self {
            Self::Autocharge(_) => None,
            Self::Booster(_) => None,
            Self::Character(_) => Some(SolLocationKind::Character),
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(_) => None,
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Ship(ship) => match ship.get_kind() {
                SolShipKind::Ship => Some(SolLocationKind::Ship),
                SolShipKind::Structure => Some(SolLocationKind::Structure),
                _ => None,
            },
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn is_on_char_root(&self) -> bool {
        match self {
            Self::Autocharge(_) => false,
            Self::Booster(_) => true,
            Self::Character(_) => false,
            Self::Charge(_) => false,
            Self::Drone(_) => false,
            Self::Fighter(_) => false,
            Self::FwEffect(_) => false,
            Self::Implant(_) => true,
            Self::Module(_) => false,
            Self::ProjEffect(_) => false,
            Self::Rig(_) => false,
            Self::Ship(_) => false,
            Self::Skill(_) => true,
            Self::Stance(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn is_on_ship_root(&self) -> bool {
        match self {
            Self::Autocharge(_) => false,
            Self::Booster(_) => false,
            Self::Character(_) => false,
            Self::Charge(_) => true, // TODO: check if it needs to be true?
            Self::Drone(_) => false,
            Self::Fighter(_) => false,
            Self::FwEffect(_) => false,
            Self::Implant(_) => false,
            Self::Module(_) => true,
            Self::ProjEffect(_) => false,
            Self::Rig(_) => true,
            Self::Ship(_) => false,
            Self::Skill(_) => false,
            Self::Stance(_) => true,
            Self::Subsystem(_) => true,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn is_on_struct_root(&self) -> bool {
        match self {
            Self::Autocharge(_) => false,
            Self::Booster(_) => false,
            Self::Character(_) => false,
            Self::Charge(_) => true, // TODO: check if it needs to be true?
            Self::Drone(_) => false,
            Self::Fighter(_) => false,
            Self::FwEffect(_) => false,
            Self::Implant(_) => false,
            Self::Module(_) => true,
            Self::ProjEffect(_) => false,
            Self::Rig(_) => true,
            Self::Ship(_) => false,
            Self::Skill(_) => false,
            Self::Stance(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn is_owner_modifiable(&self) -> bool {
        match self {
            Self::Autocharge(_) => false,
            Self::Booster(_) => false,
            Self::Character(_) => false,
            Self::Charge(_) => true,
            Self::Drone(_) => true,
            Self::Fighter(_) => true,
            Self::FwEffect(_) => false,
            Self::Implant(_) => false,
            Self::Module(_) => false,
            Self::ProjEffect(_) => false,
            Self::Rig(_) => false,
            Self::Ship(_) => false,
            Self::Skill(_) => false,
            Self::Stance(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn is_buffable(&self) -> bool {
        match self {
            Self::Autocharge(_) => false,
            Self::Booster(_) => false,
            Self::Character(_) => false,
            Self::Charge(_) => false,
            Self::Drone(_) => true,
            Self::Fighter(_) => true,
            Self::FwEffect(_) => false,
            Self::Implant(_) => false,
            Self::Module(_) => false,
            Self::ProjEffect(_) => false,
            Self::Rig(_) => false,
            Self::Ship(ship) => match ship.get_kind() {
                SolShipKind::Ship => true,
                SolShipKind::Structure => false,
                SolShipKind::Unknown => false,
            },
            Self::Skill(_) => false,
            Self::Stance(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn get_other(&self) -> Option<SolItemId> {
        match self {
            Self::Autocharge(_) => None,
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(charge) => Some(charge.get_cont_id()),
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(module) => module.get_charge_id(),
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
}
