use crate::{
    defs::SsItemId,
    ss::{item::SsItem, svc::svce_calc::SsLocType},
};

impl SsItem {
    pub(in crate::ss::svc::svce_calc) fn get_root_loc_type(&self) -> Option<SsLocType> {
        match self {
            Self::Booster(_) => None,
            Self::Character(_) => Some(SsLocType::Character),
            Self::Charge(_) => None,
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(_) => None,
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Ship(_) => Some(SsLocType::Ship),
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Structure(_) => Some(SsLocType::Structure),
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn can_have_parent_char(&self) -> bool {
        match self {
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
            Self::Structure(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn can_have_parent_ship(&self) -> bool {
        match self {
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
            Self::Structure(_) => false,
            Self::Subsystem(_) => true,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn can_have_parent_struct(&self) -> bool {
        match self {
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
            Self::Structure(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn is_owner_modifiable(&self) -> bool {
        match self {
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
            Self::Structure(_) => false,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn is_buff_modifiable(&self) -> bool {
        match self {
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
            Self::Ship(_) => true,
            Self::Skill(_) => false,
            Self::Stance(_) => false,
            Self::Structure(_) => true,
            Self::Subsystem(_) => false,
            Self::SwEffect(_) => false,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn get_other(&self) -> Option<SsItemId> {
        match self {
            Self::Booster(_) => None,
            Self::Character(_) => None,
            Self::Charge(charge) => Some(charge.cont_id),
            Self::Drone(_) => None,
            Self::Fighter(_) => None,
            Self::FwEffect(_) => None,
            Self::Implant(_) => None,
            Self::Module(module) => module.charge_item_id,
            Self::ProjEffect(_) => None,
            Self::Rig(_) => None,
            Self::Ship(_) => None,
            Self::Skill(_) => None,
            Self::Stance(_) => None,
            Self::Structure(_) => None,
            Self::Subsystem(_) => None,
            Self::SwEffect(_) => None,
        }
    }
}
