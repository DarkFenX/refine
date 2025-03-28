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
impl From<rc::FleetInfo> for HCmdResp {
    fn from(core_fleet: rc::FleetInfo) -> Self {
        Self::FleetId(core_fleet.into())
    }
}
impl From<rc::FitInfo> for HCmdResp {
    fn from(core_fit: rc::FitInfo) -> Self {
        Self::FitId(core_fit.into())
    }
}
impl From<rc::BoosterInfo> for HCmdResp {
    fn from(core_booster: rc::BoosterInfo) -> Self {
        Self::ItemIds(core_booster.into())
    }
}
impl From<rc::CharacterInfo> for HCmdResp {
    fn from(core_char: rc::CharacterInfo) -> Self {
        Self::ItemIds(core_char.into())
    }
}
impl From<rc::DroneInfo> for HCmdResp {
    fn from(core_drone: rc::DroneInfo) -> Self {
        Self::ItemIds(core_drone.into())
    }
}
impl From<rc::FighterInfo> for HCmdResp {
    fn from(core_fighter: rc::FighterInfo) -> Self {
        Self::ItemIds(core_fighter.into())
    }
}
impl From<rc::FwEffectInfo> for HCmdResp {
    fn from(core_fw_effect: rc::FwEffectInfo) -> Self {
        Self::ItemIds(core_fw_effect.into())
    }
}
impl From<rc::ImplantInfo> for HCmdResp {
    fn from(core_implant: rc::ImplantInfo) -> Self {
        Self::ItemIds(core_implant.into())
    }
}
impl From<rc::ModuleInfo> for HCmdResp {
    fn from(core_module: rc::ModuleInfo) -> Self {
        Self::ItemIds(core_module.into())
    }
}
impl From<rc::ProjEffectInfo> for HCmdResp {
    fn from(core_proj_effect: rc::ProjEffectInfo) -> Self {
        Self::ItemIds(core_proj_effect.into())
    }
}
impl From<rc::RigInfo> for HCmdResp {
    fn from(core_rig: rc::RigInfo) -> Self {
        Self::ItemIds(core_rig.into())
    }
}
impl From<rc::ServiceInfo> for HCmdResp {
    fn from(core_service: rc::ServiceInfo) -> Self {
        Self::ItemIds(core_service.into())
    }
}
impl From<rc::ShipInfo> for HCmdResp {
    fn from(core_ship: rc::ShipInfo) -> Self {
        Self::ItemIds(core_ship.into())
    }
}
impl From<rc::SkillInfo> for HCmdResp {
    fn from(core_skill: rc::SkillInfo) -> Self {
        Self::ItemIds(core_skill.into())
    }
}
impl From<rc::StanceInfo> for HCmdResp {
    fn from(core_stance: rc::StanceInfo) -> Self {
        Self::ItemIds(core_stance.into())
    }
}
impl From<rc::SubsystemInfo> for HCmdResp {
    fn from(core_subsystem: rc::SubsystemInfo) -> Self {
        Self::ItemIds(core_subsystem.into())
    }
}
impl From<rc::SwEffectInfo> for HCmdResp {
    fn from(core_sw_effect: rc::SwEffectInfo) -> Self {
        Self::ItemIds(core_sw_effect.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFleetIdResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::FleetId,
}
impl From<rc::FleetInfo> for HFleetIdResp {
    fn from(core_fleet: rc::FleetInfo) -> Self {
        Self { id: core_fleet.id }
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitIdResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::FitId,
}
impl From<rc::FitInfo> for HFitIdResp {
    fn from(core_fit: rc::FitInfo) -> Self {
        Self { id: core_fit.id }
    }
}

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HItemIdsResp {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_id: Option<rc::ItemId>,
}
impl HItemIdsResp {
    fn new(item_id: rc::ItemId, charge_info: Option<rc::ChargeInfo>) -> Self {
        Self {
            id: item_id,
            charge_id: charge_info.map(|v| v.id),
        }
    }
}
impl From<rc::BoosterInfo> for HItemIdsResp {
    fn from(core_booster: rc::BoosterInfo) -> Self {
        Self::new(core_booster.id, None)
    }
}
impl From<rc::CharacterInfo> for HItemIdsResp {
    fn from(core_char: rc::CharacterInfo) -> Self {
        Self::new(core_char.id, None)
    }
}
impl From<rc::DroneInfo> for HItemIdsResp {
    fn from(core_drone: rc::DroneInfo) -> Self {
        Self::new(core_drone.id, None)
    }
}
impl From<rc::FighterInfo> for HItemIdsResp {
    fn from(core_fighter: rc::FighterInfo) -> Self {
        Self::new(core_fighter.id, None)
    }
}
impl From<rc::FwEffectInfo> for HItemIdsResp {
    fn from(core_fw_effect: rc::FwEffectInfo) -> Self {
        Self::new(core_fw_effect.id, None)
    }
}
impl From<rc::ImplantInfo> for HItemIdsResp {
    fn from(core_implant: rc::ImplantInfo) -> Self {
        Self::new(core_implant.id, None)
    }
}
impl From<rc::ModuleInfo> for HItemIdsResp {
    fn from(core_module: rc::ModuleInfo) -> Self {
        Self::new(core_module.id, core_module.charge)
    }
}
impl From<rc::ProjEffectInfo> for HItemIdsResp {
    fn from(core_proj_effect: rc::ProjEffectInfo) -> Self {
        Self::new(core_proj_effect.id, None)
    }
}
impl From<rc::RigInfo> for HItemIdsResp {
    fn from(core_rig: rc::RigInfo) -> Self {
        Self::new(core_rig.id, None)
    }
}
impl From<rc::ServiceInfo> for HItemIdsResp {
    fn from(core_service: rc::ServiceInfo) -> Self {
        Self::new(core_service.id, None)
    }
}
impl From<rc::ShipInfo> for HItemIdsResp {
    fn from(core_ship: rc::ShipInfo) -> Self {
        Self::new(core_ship.id, None)
    }
}
impl From<rc::SkillInfo> for HItemIdsResp {
    fn from(core_skill: rc::SkillInfo) -> Self {
        Self::new(core_skill.id, None)
    }
}
impl From<rc::StanceInfo> for HItemIdsResp {
    fn from(core_stance: rc::StanceInfo) -> Self {
        Self::new(core_stance.id, None)
    }
}
impl From<rc::SubsystemInfo> for HItemIdsResp {
    fn from(core_subsystem: rc::SubsystemInfo) -> Self {
        Self::new(core_subsystem.id, None)
    }
}
impl From<rc::SwEffectInfo> for HItemIdsResp {
    fn from(core_sw_effect: rc::SwEffectInfo) -> Self {
        Self::new(core_sw_effect.id, None)
    }
}
