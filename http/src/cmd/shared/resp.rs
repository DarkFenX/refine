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
impl From<rc::SolFleetInfo> for HCmdResp {
    fn from(core_fleet: rc::SolFleetInfo) -> Self {
        HCmdResp::FleetId(core_fleet.into())
    }
}
impl From<rc::SolFitInfo> for HCmdResp {
    fn from(core_fit: rc::SolFitInfo) -> Self {
        HCmdResp::FitId(core_fit.into())
    }
}
impl From<rc::SolCharacterInfo> for HCmdResp {
    fn from(core_char: rc::SolCharacterInfo) -> Self {
        HCmdResp::ItemIds(core_char.into())
    }
}
impl From<rc::SolSkillInfo> for HCmdResp {
    fn from(core_skill: rc::SolSkillInfo) -> Self {
        HCmdResp::ItemIds(core_skill.into())
    }
}
impl From<rc::SolImplantInfo> for HCmdResp {
    fn from(core_implant: rc::SolImplantInfo) -> Self {
        HCmdResp::ItemIds(core_implant.into())
    }
}
impl From<rc::SolBoosterInfo> for HCmdResp {
    fn from(core_booster: rc::SolBoosterInfo) -> Self {
        HCmdResp::ItemIds(core_booster.into())
    }
}
impl From<rc::SolShipInfo> for HCmdResp {
    fn from(core_ship: rc::SolShipInfo) -> Self {
        HCmdResp::ItemIds(core_ship.into())
    }
}
impl From<rc::SolStanceInfo> for HCmdResp {
    fn from(core_stance: rc::SolStanceInfo) -> Self {
        HCmdResp::ItemIds(core_stance.into())
    }
}
impl From<rc::SolSubsystemInfo> for HCmdResp {
    fn from(core_subsystem: rc::SolSubsystemInfo) -> Self {
        HCmdResp::ItemIds(core_subsystem.into())
    }
}
impl From<rc::SolModuleInfo> for HCmdResp {
    fn from(core_module: rc::SolModuleInfo) -> Self {
        HCmdResp::ItemIds(core_module.into())
    }
}
impl From<rc::SolRigInfo> for HCmdResp {
    fn from(core_rig: rc::SolRigInfo) -> Self {
        HCmdResp::ItemIds(core_rig.into())
    }
}
impl From<rc::SolDroneInfo> for HCmdResp {
    fn from(core_drone: rc::SolDroneInfo) -> Self {
        HCmdResp::ItemIds(core_drone.into())
    }
}
impl From<rc::SolFighterInfo> for HCmdResp {
    fn from(core_fighter: rc::SolFighterInfo) -> Self {
        HCmdResp::ItemIds(core_fighter.into())
    }
}
impl From<rc::SolSwEffectInfo> for HCmdResp {
    fn from(core_sw_effect: rc::SolSwEffectInfo) -> Self {
        HCmdResp::ItemIds(core_sw_effect.into())
    }
}
impl From<rc::SolFwEffectInfo> for HCmdResp {
    fn from(core_fw_effect: rc::SolFwEffectInfo) -> Self {
        HCmdResp::ItemIds(core_fw_effect.into())
    }
}
impl From<rc::SolProjEffectInfo> for HCmdResp {
    fn from(core_proj_effect: rc::SolProjEffectInfo) -> Self {
        HCmdResp::ItemIds(core_proj_effect.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetIdResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::SolFleetId,
}
impl HFleetIdResp {
    fn new(fleet_id: rc::SolFleetId) -> Self {
        Self { id: fleet_id }
    }
}
impl From<rc::SolFleetInfo> for HFleetIdResp {
    fn from(core_fleet: rc::SolFleetInfo) -> Self {
        HFleetIdResp::new(core_fleet.id)
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitIdResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::SolFitId,
}
impl HFitIdResp {
    fn new(fit_id: rc::SolFitId) -> Self {
        Self { id: fit_id }
    }
}
impl From<rc::SolFitInfo> for HFitIdResp {
    fn from(core_fit: rc::SolFitInfo) -> Self {
        HFitIdResp::new(core_fit.id)
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HItemIdsResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::SolItemId,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_id: Option<rc::SolItemId>,
}
impl HItemIdsResp {
    fn new(item_id: rc::SolItemId, charge_info: Option<rc::SolChargeInfo>) -> Self {
        Self {
            id: item_id,
            charge_id: charge_info.map(|v| v.id),
        }
    }
}
impl From<rc::SolCharacterInfo> for HItemIdsResp {
    fn from(core_char: rc::SolCharacterInfo) -> Self {
        HItemIdsResp::new(core_char.id, None)
    }
}
impl From<rc::SolSkillInfo> for HItemIdsResp {
    fn from(core_skill: rc::SolSkillInfo) -> Self {
        HItemIdsResp::new(core_skill.id, None)
    }
}
impl From<rc::SolImplantInfo> for HItemIdsResp {
    fn from(core_implant: rc::SolImplantInfo) -> Self {
        HItemIdsResp::new(core_implant.id, None)
    }
}
impl From<rc::SolBoosterInfo> for HItemIdsResp {
    fn from(core_booster: rc::SolBoosterInfo) -> Self {
        HItemIdsResp::new(core_booster.id, None)
    }
}
impl From<rc::SolShipInfo> for HItemIdsResp {
    fn from(core_ship: rc::SolShipInfo) -> Self {
        HItemIdsResp::new(core_ship.id, None)
    }
}
impl From<rc::SolStanceInfo> for HItemIdsResp {
    fn from(core_stance: rc::SolStanceInfo) -> Self {
        HItemIdsResp::new(core_stance.id, None)
    }
}
impl From<rc::SolSubsystemInfo> for HItemIdsResp {
    fn from(core_subsystem: rc::SolSubsystemInfo) -> Self {
        HItemIdsResp::new(core_subsystem.id, None)
    }
}
impl From<rc::SolModuleInfo> for HItemIdsResp {
    fn from(core_module: rc::SolModuleInfo) -> Self {
        HItemIdsResp::new(core_module.id, core_module.charge)
    }
}
impl From<rc::SolRigInfo> for HItemIdsResp {
    fn from(core_rig: rc::SolRigInfo) -> Self {
        HItemIdsResp::new(core_rig.id, None)
    }
}
impl From<rc::SolDroneInfo> for HItemIdsResp {
    fn from(core_drone: rc::SolDroneInfo) -> Self {
        HItemIdsResp::new(core_drone.id, None)
    }
}
impl From<rc::SolFighterInfo> for HItemIdsResp {
    fn from(core_fighter: rc::SolFighterInfo) -> Self {
        HItemIdsResp::new(core_fighter.id, None)
    }
}
impl From<rc::SolSwEffectInfo> for HItemIdsResp {
    fn from(core_sw_effect: rc::SolSwEffectInfo) -> Self {
        HItemIdsResp::new(core_sw_effect.id, None)
    }
}
impl From<rc::SolFwEffectInfo> for HItemIdsResp {
    fn from(core_fw_effect: rc::SolFwEffectInfo) -> Self {
        HItemIdsResp::new(core_fw_effect.id, None)
    }
}
impl From<rc::SolProjEffectInfo> for HItemIdsResp {
    fn from(core_proj_effect: rc::SolProjEffectInfo) -> Self {
        HItemIdsResp::new(core_proj_effect.id, None)
    }
}
