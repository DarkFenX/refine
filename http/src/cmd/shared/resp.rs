use rc::ItemCommon;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HCmdResp {
    NoData,
    FitId(HFitIdResp),
    FleetId(HFleetIdResp),
    ItemIds(HItemIdsResp),
}
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

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetIdResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::FleetId,
}
impl From<rc::FleetMut<'_>> for HFleetIdResp {
    fn from(core_fleet: rc::FleetMut) -> Self {
        Self {
            id: core_fleet.get_fleet_id(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitIdResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::FitId,
}
impl From<rc::FitMut<'_>> for HFitIdResp {
    fn from(core_fit: rc::FitMut) -> Self {
        Self {
            id: core_fit.get_fit_id(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HItemIdsResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) charge: Option<rc::ItemId>,
}
impl From<rc::AutochargeMut<'_>> for HItemIdsResp {
    fn from(core_autocharge: rc::AutochargeMut) -> Self {
        Self {
            id: core_autocharge.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::BoosterMut<'_>> for HItemIdsResp {
    fn from(core_booster: rc::BoosterMut) -> Self {
        Self {
            id: core_booster.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::CharacterMut<'_>> for HItemIdsResp {
    fn from(core_character: rc::CharacterMut) -> Self {
        Self {
            id: core_character.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::ChargeMut<'_>> for HItemIdsResp {
    fn from(core_charge: rc::ChargeMut) -> Self {
        Self {
            id: core_charge.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::DroneMut<'_>> for HItemIdsResp {
    fn from(core_drone: rc::DroneMut) -> Self {
        Self {
            id: core_drone.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::FighterMut<'_>> for HItemIdsResp {
    fn from(core_fighter: rc::FighterMut) -> Self {
        Self {
            id: core_fighter.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::FwEffectMut<'_>> for HItemIdsResp {
    fn from(core_fw_effect: rc::FwEffectMut) -> Self {
        Self {
            id: core_fw_effect.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::ImplantMut<'_>> for HItemIdsResp {
    fn from(core_implant: rc::ImplantMut) -> Self {
        Self {
            id: core_implant.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::ModuleMut<'_>> for HItemIdsResp {
    fn from(mut core_module: rc::ModuleMut) -> Self {
        Self {
            id: core_module.get_item_id(),
            charge: core_module
                .get_charge_mut()
                .map(|core_charge| core_charge.get_item_id()),
        }
    }
}
impl From<rc::ProjEffectMut<'_>> for HItemIdsResp {
    fn from(core_proj_effect: rc::ProjEffectMut) -> Self {
        Self {
            id: core_proj_effect.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::RigMut<'_>> for HItemIdsResp {
    fn from(core_rig: rc::RigMut) -> Self {
        Self {
            id: core_rig.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::ServiceMut<'_>> for HItemIdsResp {
    fn from(core_service: rc::ServiceMut) -> Self {
        Self {
            id: core_service.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::ShipMut<'_>> for HItemIdsResp {
    fn from(core_ship: rc::ShipMut) -> Self {
        Self {
            id: core_ship.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::SkillMut<'_>> for HItemIdsResp {
    fn from(core_skill: rc::SkillMut) -> Self {
        Self {
            id: core_skill.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::StanceMut<'_>> for HItemIdsResp {
    fn from(core_stance: rc::StanceMut) -> Self {
        Self {
            id: core_stance.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::SubsystemMut<'_>> for HItemIdsResp {
    fn from(core_subsystem: rc::SubsystemMut) -> Self {
        Self {
            id: core_subsystem.get_item_id(),
            charge: None,
        }
    }
}
impl From<rc::SwEffectMut<'_>> for HItemIdsResp {
    fn from(core_sw_effect: rc::SwEffectMut) -> Self {
        Self {
            id: core_sw_effect.get_item_id(),
            charge: None,
        }
    }
}
