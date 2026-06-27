use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HCmdResp {
    CreatedFitId(HCreatedFitIdResp),
    CreatedFleetId(HCreatedFleetIdResp),
    CreatedItemIds(HCreatedItemIdsResp),
    ChangedItemIds(HChangedItemIdsResp),
    // This variant has an empty value just to serialize as an object, not as null
    NoData {},
}

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HCreatedFleetIdResp {
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) id: rc::FleetId,
}

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HCreatedFitIdResp {
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) id: rc::FitId,
}

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HCreatedItemIdsResp {
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) charge_id: Option<rc::ItemId>,
}

#[serde_as]
#[derive(Default, Serialize)]
pub(crate) struct HChangedItemIdsResp {
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) charge_id: Option<rc::ItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<()> for HCmdResp {
    fn from(_: ()) -> Self {
        HCmdResp::NoData {}
    }
}
impl From<HCreatedFitIdResp> for HCmdResp {
    fn from(resp: HCreatedFitIdResp) -> Self {
        HCmdResp::CreatedFitId(resp)
    }
}
impl From<HCreatedFleetIdResp> for HCmdResp {
    fn from(resp: HCreatedFleetIdResp) -> Self {
        HCmdResp::CreatedFleetId(resp)
    }
}
impl From<HCreatedItemIdsResp> for HCmdResp {
    fn from(resp: HCreatedItemIdsResp) -> Self {
        HCmdResp::CreatedItemIds(resp)
    }
}
impl From<HChangedItemIdsResp> for HCmdResp {
    fn from(resp: HChangedItemIdsResp) -> Self {
        HCmdResp::ChangedItemIds(resp)
    }
}

impl HCreatedFleetIdResp {
    pub(in crate::cmd) fn from_core_fleet(core_fleet: rc::FleetMut) -> Self {
        Self {
            id: core_fleet.get_fleet_id(),
        }
    }
}

impl HCreatedFitIdResp {
    pub(in crate::cmd) fn from_core_fit(core_fit: rc::FitMut) -> Self {
        Self {
            id: core_fit.get_fit_id(),
        }
    }
}

impl HCreatedItemIdsResp {
    pub(in crate::cmd) fn from_core_booster(core_booster: rc::BoosterMut) -> Self {
        Self {
            id: core_booster.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_character(core_character: rc::CharacterMut) -> Self {
        Self {
            id: core_character.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_drone(core_drone: rc::DroneMut) -> Self {
        Self {
            id: core_drone.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_fighter(core_fighter: rc::FighterMut) -> Self {
        Self {
            id: core_fighter.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_fw_effect(core_fw_effect: rc::FwEffectMut) -> Self {
        Self {
            id: core_fw_effect.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_implant(core_implant: rc::ImplantMut) -> Self {
        Self {
            id: core_implant.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_module(core_module: rc::ModuleMut) -> Self {
        Self {
            id: core_module.get_item_id(),
            charge_id: core_module.get_charge().map(|core_charge| core_charge.get_item_id()),
        }
    }
    pub(in crate::cmd) fn from_core_proj_effect(core_proj_effect: rc::ProjEffectMut) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_rig(core_rig: rc::RigMut) -> Self {
        Self {
            id: core_rig.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_service(core_service: rc::ServiceMut) -> Self {
        Self {
            id: core_service.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_ship(core_ship: rc::ShipMut) -> Self {
        Self {
            id: core_ship.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_skill(core_skill: rc::SkillMut) -> Self {
        Self {
            id: core_skill.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_stance(core_stance: rc::StanceMut) -> Self {
        Self {
            id: core_stance.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_subsystem(core_subsystem: rc::SubsystemMut) -> Self {
        Self {
            id: core_subsystem.get_item_id(),
            charge_id: None,
        }
    }
    pub(in crate::cmd) fn from_core_sw_effect(core_sw_effect: rc::SwEffectMut) -> Self {
        Self {
            id: core_sw_effect.get_item_id(),
            charge_id: None,
        }
    }
}

impl HChangedItemIdsResp {
    pub(in crate::cmd) fn from_core_charge(core_charge: rc::ChargeMut) -> Self {
        Self {
            charge_id: Some(core_charge.get_item_id()),
        }
    }
}
