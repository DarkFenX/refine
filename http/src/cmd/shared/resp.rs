use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum HCmdResp {
    NoData,
    FitId(HFitIdResp),
    FleetId(HFleetIdResp),
    ItemIds(HItemIdsResp),
}

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFleetIdResp {
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) id: rc::FleetId,
}

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFitIdResp {
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) id: rc::FitId,
}

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HItemIdsResp {
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) charge: Option<rc::ItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<()> for HCmdResp {
    fn from(_: ()) -> Self {
        HCmdResp::NoData
    }
}
impl From<HFitIdResp> for HCmdResp {
    fn from(resp: HFitIdResp) -> Self {
        HCmdResp::FitId(resp)
    }
}
impl From<HFleetIdResp> for HCmdResp {
    fn from(resp: HFleetIdResp) -> Self {
        HCmdResp::FleetId(resp)
    }
}
impl From<HItemIdsResp> for HCmdResp {
    fn from(resp: HItemIdsResp) -> Self {
        HCmdResp::ItemIds(resp)
    }
}

impl HFleetIdResp {
    pub(in crate::cmd) fn from_core_fleet(core_fleet: rc::FleetMut) -> Self {
        Self {
            id: core_fleet.get_fleet_id(),
        }
    }
}

impl HFitIdResp {
    pub(in crate::cmd) fn from_core_fit(core_fit: rc::FitMut) -> Self {
        Self {
            id: core_fit.get_fit_id(),
        }
    }
}

impl HItemIdsResp {
    pub(in crate::cmd) fn from_core_autocharge(core_autocharge: rc::AutochargeMut) -> Self {
        Self {
            id: core_autocharge.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_booster(core_booster: rc::BoosterMut) -> Self {
        Self {
            id: core_booster.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_character(core_character: rc::CharacterMut) -> Self {
        Self {
            id: core_character.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_charge(core_charge: rc::ChargeMut) -> Self {
        Self {
            id: core_charge.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_drone(core_drone: rc::DroneMut) -> Self {
        Self {
            id: core_drone.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_fighter(core_fighter: rc::FighterMut) -> Self {
        Self {
            id: core_fighter.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_fw_effect(core_fw_effect: rc::FwEffectMut) -> Self {
        Self {
            id: core_fw_effect.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_implant(core_implant: rc::ImplantMut) -> Self {
        Self {
            id: core_implant.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_module(core_module: rc::ModuleMut) -> Self {
        Self {
            id: core_module.get_item_id(),
            charge: core_module.get_charge().map(|core_charge| core_charge.get_item_id()),
        }
    }
    pub(in crate::cmd) fn from_core_proj_effect(core_proj_effect: rc::ProjEffectMut) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_rig(core_rig: rc::RigMut) -> Self {
        Self {
            id: core_rig.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_service(core_service: rc::ServiceMut) -> Self {
        Self {
            id: core_service.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_ship(core_ship: rc::ShipMut) -> Self {
        Self {
            id: core_ship.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_skill(core_skill: rc::SkillMut) -> Self {
        Self {
            id: core_skill.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_stance(core_stance: rc::StanceMut) -> Self {
        Self {
            id: core_stance.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_subsystem(core_subsystem: rc::SubsystemMut) -> Self {
        Self {
            id: core_subsystem.get_item_id(),
            charge: None,
        }
    }
    pub(in crate::cmd) fn from_core_sw_effect(core_sw_effect: rc::SwEffectMut) -> Self {
        Self {
            id: core_sw_effect.get_item_id(),
            charge: None,
        }
    }
}
