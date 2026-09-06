use rc::ItemCommon;

use crate::{FitId, FleetId, ItemId};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct AddedFleetIdResp {
    pub fleet_id: FleetId,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct AddedFitIdResp {
    pub fit_id: FitId,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct AddedItemIdsResp {
    pub item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge_item_id: Option<ItemId>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone, Default)]
pub struct ChangedItemIdsResp {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge_item_id: Option<ItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddedFleetIdResp {
    pub(in crate::ctl::core) fn from_core_fleet(core_fleet: rc::FleetMut) -> Self {
        Self {
            fleet_id: core_fleet.get_fleet_id(),
        }
    }
}

impl AddedFitIdResp {
    pub(in crate::ctl::core) fn from_core_fit(core_fit: rc::FitMut) -> Self {
        Self {
            fit_id: core_fit.get_fit_id(),
        }
    }
}

impl AddedItemIdsResp {
    pub(in crate::ctl::core) fn from_core_item(core_item: rc::ItemMut) -> Self {
        Self {
            item_id: core_item.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_booster(core_booster: rc::BoosterMut) -> Self {
        Self {
            item_id: core_booster.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_character(core_character: rc::CharacterMut) -> Self {
        Self {
            item_id: core_character.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_drone(core_drone: rc::DroneMut) -> Self {
        Self {
            item_id: core_drone.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_fighter(core_fighter: rc::FighterMut) -> Self {
        Self {
            item_id: core_fighter.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_fw_effect(core_fw_effect: rc::FwEffectMut) -> Self {
        Self {
            item_id: core_fw_effect.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_implant(core_implant: rc::ImplantMut) -> Self {
        Self {
            item_id: core_implant.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_module(core_module: rc::ModuleMut) -> Self {
        Self {
            item_id: core_module.get_item_id(),
            charge_item_id: core_module.get_charge().map(|core_charge| core_charge.get_item_id()),
        }
    }
    pub(in crate::ctl::core) fn from_core_proj_effect(core_proj_effect: rc::ProjEffectMut) -> Self {
        Self {
            item_id: core_proj_effect.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_rig(core_rig: rc::RigMut) -> Self {
        Self {
            item_id: core_rig.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_service(core_service: rc::ServiceMut) -> Self {
        Self {
            item_id: core_service.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_ship(core_ship: rc::ShipMut) -> Self {
        Self {
            item_id: core_ship.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_skill(core_skill: rc::SkillMut) -> Self {
        Self {
            item_id: core_skill.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_stance(core_stance: rc::StanceMut) -> Self {
        Self {
            item_id: core_stance.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_subsystem(core_subsystem: rc::SubsystemMut) -> Self {
        Self {
            item_id: core_subsystem.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::ctl::core) fn from_core_sw_effect(core_sw_effect: rc::SwEffectMut) -> Self {
        Self {
            item_id: core_sw_effect.get_item_id(),
            charge_item_id: None,
        }
    }
}

impl ChangedItemIdsResp {
    pub(in crate::ctl::core) fn from_core_charge(core_charge: rc::ChargeMut) -> Self {
        Self {
            charge_item_id: Some(core_charge.get_item_id()),
        }
    }
}
