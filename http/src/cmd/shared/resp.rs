#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HCmdResp {
    NoData,
    FitId(HFitIdResp),
    FleetId(HFleetIdResp),
    ItemIds(HItemIdsResp),
}
impl From<rc::SsFleetInfo> for HCmdResp {
    fn from(core_fleet: rc::SsFleetInfo) -> Self {
        HCmdResp::FleetId(core_fleet.into())
    }
}
impl From<rc::SsFitInfo> for HCmdResp {
    fn from(core_fit: rc::SsFitInfo) -> Self {
        HCmdResp::FitId(core_fit.into())
    }
}
impl From<rc::SsCharacterInfo> for HCmdResp {
    fn from(core_char: rc::SsCharacterInfo) -> Self {
        HCmdResp::ItemIds(core_char.into())
    }
}
impl From<rc::SsSkillInfo> for HCmdResp {
    fn from(core_skill: rc::SsSkillInfo) -> Self {
        HCmdResp::ItemIds(core_skill.into())
    }
}
impl From<rc::SsImplantInfo> for HCmdResp {
    fn from(core_implant: rc::SsImplantInfo) -> Self {
        HCmdResp::ItemIds(core_implant.into())
    }
}
impl From<rc::SsBoosterInfo> for HCmdResp {
    fn from(core_booster: rc::SsBoosterInfo) -> Self {
        HCmdResp::ItemIds(core_booster.into())
    }
}
impl From<rc::SsShipInfo> for HCmdResp {
    fn from(core_ship: rc::SsShipInfo) -> Self {
        HCmdResp::ItemIds(core_ship.into())
    }
}
impl From<rc::SsStructureInfo> for HCmdResp {
    fn from(core_structure: rc::SsStructureInfo) -> Self {
        HCmdResp::ItemIds(core_structure.into())
    }
}
impl From<rc::SsStanceInfo> for HCmdResp {
    fn from(core_stance: rc::SsStanceInfo) -> Self {
        HCmdResp::ItemIds(core_stance.into())
    }
}
impl From<rc::SsSubsystemInfo> for HCmdResp {
    fn from(core_subsystem: rc::SsSubsystemInfo) -> Self {
        HCmdResp::ItemIds(core_subsystem.into())
    }
}
impl From<rc::SsModuleInfo> for HCmdResp {
    fn from(core_module: rc::SsModuleInfo) -> Self {
        HCmdResp::ItemIds(core_module.into())
    }
}
impl From<rc::SsRigInfo> for HCmdResp {
    fn from(core_rig: rc::SsRigInfo) -> Self {
        HCmdResp::ItemIds(core_rig.into())
    }
}
impl From<rc::SsDroneInfo> for HCmdResp {
    fn from(core_drone: rc::SsDroneInfo) -> Self {
        HCmdResp::ItemIds(core_drone.into())
    }
}
impl From<rc::SsFighterInfo> for HCmdResp {
    fn from(core_fighter: rc::SsFighterInfo) -> Self {
        HCmdResp::ItemIds(core_fighter.into())
    }
}
impl From<rc::SsSwEffectInfo> for HCmdResp {
    fn from(core_sw_effect: rc::SsSwEffectInfo) -> Self {
        HCmdResp::ItemIds(core_sw_effect.into())
    }
}
impl From<rc::SsFwEffectInfo> for HCmdResp {
    fn from(core_fw_effect: rc::SsFwEffectInfo) -> Self {
        HCmdResp::ItemIds(core_fw_effect.into())
    }
}
impl From<rc::SsProjEffectInfo> for HCmdResp {
    fn from(core_proj_effect: rc::SsProjEffectInfo) -> Self {
        HCmdResp::ItemIds(core_proj_effect.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetIdResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::SsFleetId,
}
impl HFleetIdResp {
    fn new(fleet_id: rc::SsFleetId) -> Self {
        Self { id: fleet_id }
    }
}
impl From<rc::SsFleetInfo> for HFleetIdResp {
    fn from(core_fleet: rc::SsFleetInfo) -> Self {
        HFleetIdResp::new(core_fleet.id)
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitIdResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::SsFitId,
}
impl HFitIdResp {
    fn new(fit_id: rc::SsFitId) -> Self {
        Self { id: fit_id }
    }
}
impl From<rc::SsFitInfo> for HFitIdResp {
    fn from(core_fit: rc::SsFitInfo) -> Self {
        HFitIdResp::new(core_fit.id)
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HItemIdsResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::SsItemId,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_id: Option<rc::SsItemId>,
}
impl HItemIdsResp {
    fn new(item_id: rc::SsItemId, charge_info: Option<rc::SsChargeInfo>) -> Self {
        Self {
            id: item_id,
            charge_id: charge_info.map(|v| v.id),
        }
    }
}
impl From<rc::SsCharacterInfo> for HItemIdsResp {
    fn from(core_char: rc::SsCharacterInfo) -> Self {
        HItemIdsResp::new(core_char.id, None)
    }
}
impl From<rc::SsSkillInfo> for HItemIdsResp {
    fn from(core_skill: rc::SsSkillInfo) -> Self {
        HItemIdsResp::new(core_skill.id, None)
    }
}
impl From<rc::SsImplantInfo> for HItemIdsResp {
    fn from(core_implant: rc::SsImplantInfo) -> Self {
        HItemIdsResp::new(core_implant.id, None)
    }
}
impl From<rc::SsBoosterInfo> for HItemIdsResp {
    fn from(core_booster: rc::SsBoosterInfo) -> Self {
        HItemIdsResp::new(core_booster.id, None)
    }
}
impl From<rc::SsShipInfo> for HItemIdsResp {
    fn from(core_ship: rc::SsShipInfo) -> Self {
        HItemIdsResp::new(core_ship.id, None)
    }
}
impl From<rc::SsStructureInfo> for HItemIdsResp {
    fn from(core_structure: rc::SsStructureInfo) -> Self {
        HItemIdsResp::new(core_structure.id, None)
    }
}
impl From<rc::SsStanceInfo> for HItemIdsResp {
    fn from(core_stance: rc::SsStanceInfo) -> Self {
        HItemIdsResp::new(core_stance.id, None)
    }
}
impl From<rc::SsSubsystemInfo> for HItemIdsResp {
    fn from(core_subsystem: rc::SsSubsystemInfo) -> Self {
        HItemIdsResp::new(core_subsystem.id, None)
    }
}
impl From<rc::SsModuleInfo> for HItemIdsResp {
    fn from(core_module: rc::SsModuleInfo) -> Self {
        HItemIdsResp::new(core_module.id, core_module.ss_charge_info)
    }
}
impl From<rc::SsRigInfo> for HItemIdsResp {
    fn from(core_rig: rc::SsRigInfo) -> Self {
        HItemIdsResp::new(core_rig.id, None)
    }
}
impl From<rc::SsDroneInfo> for HItemIdsResp {
    fn from(core_drone: rc::SsDroneInfo) -> Self {
        HItemIdsResp::new(core_drone.id, None)
    }
}
impl From<rc::SsFighterInfo> for HItemIdsResp {
    fn from(core_fighter: rc::SsFighterInfo) -> Self {
        HItemIdsResp::new(core_fighter.id, None)
    }
}
impl From<rc::SsSwEffectInfo> for HItemIdsResp {
    fn from(core_sw_effect: rc::SsSwEffectInfo) -> Self {
        HItemIdsResp::new(core_sw_effect.id, None)
    }
}
impl From<rc::SsFwEffectInfo> for HItemIdsResp {
    fn from(core_fw_effect: rc::SsFwEffectInfo) -> Self {
        HItemIdsResp::new(core_fw_effect.id, None)
    }
}
impl From<rc::SsProjEffectInfo> for HItemIdsResp {
    fn from(core_proj_effect: rc::SsProjEffectInfo) -> Self {
        HItemIdsResp::new(core_proj_effect.id, None)
    }
}
