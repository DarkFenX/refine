use rc::ItemCommon;

use crate::{FitId, FleetId, ItemId};

pub enum CmdResp {
    AddedFleetId(AddedFleetIdResp),
    AddedFitId(AddedFitIdResp),
    AddedItemIds(AddedItemIdsResp),
    ChangedItemIds(ChangedItemIdsResp),
    // TODO: this variant serializes into null in JSON, but NoData {} is ugly, write custom ser impl
    NoData,
}
impl CmdResp {
    pub fn get_fleet_id(&self) -> Option<FleetId> {
        match self {
            Self::AddedFleetId(resp) => Some(resp.fleet_id),
            _ => None,
        }
    }
    pub fn get_fit_id(&self) -> Option<FitId> {
        match self {
            Self::AddedFitId(resp) => Some(resp.fit_id),
            _ => None,
        }
    }
    pub fn get_item_id(&self) -> Option<ItemId> {
        match self {
            Self::AddedItemIds(resp) => Some(resp.item_id),
            _ => None,
        }
    }
    pub fn get_charge_item_id(&self) -> Option<ItemId> {
        match self {
            Self::AddedItemIds(resp) => resp.charge_item_id,
            Self::ChangedItemIds(resp) => resp.charge_item_id,
            _ => None,
        }
    }
}

pub struct AddedFleetIdResp {
    pub fleet_id: FleetId,
}

pub struct AddedFitIdResp {
    pub fit_id: FitId,
}

pub struct AddedItemIdsResp {
    pub item_id: ItemId,
    pub charge_item_id: Option<ItemId>,
}

#[derive(Default)]
pub struct ChangedItemIdsResp {
    pub charge_item_id: Option<ItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<()> for CmdResp {
    fn from(_: ()) -> Self {
        CmdResp::NoData
    }
}
impl From<AddedFitIdResp> for CmdResp {
    fn from(resp: AddedFitIdResp) -> Self {
        CmdResp::AddedFitId(resp)
    }
}
impl From<AddedFleetIdResp> for CmdResp {
    fn from(resp: AddedFleetIdResp) -> Self {
        CmdResp::AddedFleetId(resp)
    }
}
impl From<AddedItemIdsResp> for CmdResp {
    fn from(resp: AddedItemIdsResp) -> Self {
        CmdResp::AddedItemIds(resp)
    }
}
impl From<ChangedItemIdsResp> for CmdResp {
    fn from(resp: ChangedItemIdsResp) -> Self {
        CmdResp::ChangedItemIds(resp)
    }
}

impl AddedFleetIdResp {
    pub(in crate::cmd) fn from_core_fleet(core_fleet: rc::FleetMut) -> Self {
        Self {
            fleet_id: core_fleet.get_fleet_id(),
        }
    }
}

impl AddedFitIdResp {
    pub(in crate::cmd) fn from_core_fit(core_fit: rc::FitMut) -> Self {
        Self {
            fit_id: core_fit.get_fit_id(),
        }
    }
}

impl AddedItemIdsResp {
    pub(in crate::cmd) fn from_core_booster(core_booster: rc::BoosterMut) -> Self {
        Self {
            item_id: core_booster.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_character(core_character: rc::CharacterMut) -> Self {
        Self {
            item_id: core_character.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_drone(core_drone: rc::DroneMut) -> Self {
        Self {
            item_id: core_drone.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_fighter(core_fighter: rc::FighterMut) -> Self {
        Self {
            item_id: core_fighter.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_fw_effect(core_fw_effect: rc::FwEffectMut) -> Self {
        Self {
            item_id: core_fw_effect.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_implant(core_implant: rc::ImplantMut) -> Self {
        Self {
            item_id: core_implant.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_module(core_module: rc::ModuleMut) -> Self {
        Self {
            item_id: core_module.get_item_id(),
            charge_item_id: core_module.get_charge().map(|core_charge| core_charge.get_item_id()),
        }
    }
    pub(in crate::cmd) fn from_core_proj_effect(core_proj_effect: rc::ProjEffectMut) -> Self {
        Self {
            item_id: core_proj_effect.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_rig(core_rig: rc::RigMut) -> Self {
        Self {
            item_id: core_rig.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_service(core_service: rc::ServiceMut) -> Self {
        Self {
            item_id: core_service.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_ship(core_ship: rc::ShipMut) -> Self {
        Self {
            item_id: core_ship.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_skill(core_skill: rc::SkillMut) -> Self {
        Self {
            item_id: core_skill.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_stance(core_stance: rc::StanceMut) -> Self {
        Self {
            item_id: core_stance.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_subsystem(core_subsystem: rc::SubsystemMut) -> Self {
        Self {
            item_id: core_subsystem.get_item_id(),
            charge_item_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_sw_effect(core_sw_effect: rc::SwEffectMut) -> Self {
        Self {
            item_id: core_sw_effect.get_item_id(),
            charge_item_id: None,
        }
    }
}

impl ChangedItemIdsResp {
    pub(in crate::cmd) fn from_core_charge(core_charge: rc::ChargeMut) -> Self {
        Self {
            charge_item_id: Some(core_charge.get_item_id()),
        }
    }
}
