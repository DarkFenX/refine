#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HCmdResp {
    NoData,
    ItemIds(HItemIdsResp),
}
impl From<rc::SsCharacterInfo> for HCmdResp {
    fn from(ss_char_info: rc::SsCharacterInfo) -> Self {
        HCmdResp::ItemIds(ss_char_info.into())
    }
}
impl From<rc::SsSkillInfo> for HCmdResp {
    fn from(ss_skill_info: rc::SsSkillInfo) -> Self {
        HCmdResp::ItemIds(ss_skill_info.into())
    }
}
impl From<rc::SsImplantInfo> for HCmdResp {
    fn from(ss_implant_info: rc::SsImplantInfo) -> Self {
        HCmdResp::ItemIds(ss_implant_info.into())
    }
}
impl From<rc::SsBoosterInfo> for HCmdResp {
    fn from(ss_booster_info: rc::SsBoosterInfo) -> Self {
        HCmdResp::ItemIds(ss_booster_info.into())
    }
}
impl From<rc::SsShipInfo> for HCmdResp {
    fn from(ss_ship_info: rc::SsShipInfo) -> Self {
        HCmdResp::ItemIds(ss_ship_info.into())
    }
}
impl From<rc::SsStructureInfo> for HCmdResp {
    fn from(ss_structure_info: rc::SsStructureInfo) -> Self {
        HCmdResp::ItemIds(ss_structure_info.into())
    }
}
impl From<rc::SsStanceInfo> for HCmdResp {
    fn from(ss_stance_info: rc::SsStanceInfo) -> Self {
        HCmdResp::ItemIds(ss_stance_info.into())
    }
}
impl From<rc::SsSubsystemInfo> for HCmdResp {
    fn from(ss_subsystem_info: rc::SsSubsystemInfo) -> Self {
        HCmdResp::ItemIds(ss_subsystem_info.into())
    }
}
impl From<rc::SsModuleInfo> for HCmdResp {
    fn from(ss_module_info: rc::SsModuleInfo) -> Self {
        HCmdResp::ItemIds(ss_module_info.into())
    }
}
impl From<rc::SsRigInfo> for HCmdResp {
    fn from(ss_rig_info: rc::SsRigInfo) -> Self {
        HCmdResp::ItemIds(ss_rig_info.into())
    }
}
impl From<rc::SsDroneInfo> for HCmdResp {
    fn from(ss_drone_info: rc::SsDroneInfo) -> Self {
        HCmdResp::ItemIds(ss_drone_info.into())
    }
}
impl From<rc::SsFighterInfo> for HCmdResp {
    fn from(ss_fighter_info: rc::SsFighterInfo) -> Self {
        HCmdResp::ItemIds(ss_fighter_info.into())
    }
}
impl From<rc::SsSwEffectInfo> for HCmdResp {
    fn from(ss_sw_effect_info: rc::SsSwEffectInfo) -> Self {
        HCmdResp::ItemIds(ss_sw_effect_info.into())
    }
}
impl From<rc::SsFwEffectInfo> for HCmdResp {
    fn from(ss_fw_effect_info: rc::SsFwEffectInfo) -> Self {
        HCmdResp::ItemIds(ss_fw_effect_info.into())
    }
}
impl From<rc::SsProjEffectInfo> for HCmdResp {
    fn from(ss_proj_effect_info: rc::SsProjEffectInfo) -> Self {
        HCmdResp::ItemIds(ss_proj_effect_info.into())
    }
}

#[derive(serde::Serialize)]
pub(crate) struct HItemIdsResp {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_id: Option<String>,
}
impl HItemIdsResp {
    fn new(item_id: rc::SsItemId, charge_info: Option<rc::SsChargeInfo>) -> Self {
        Self {
            id: item_id.to_string(),
            charge_id: charge_info.map(|v| v.id.to_string()),
        }
    }
}
impl From<rc::SsCharacterInfo> for HItemIdsResp {
    fn from(ss_char_info: rc::SsCharacterInfo) -> Self {
        HItemIdsResp::new(ss_char_info.id, None)
    }
}
impl From<rc::SsSkillInfo> for HItemIdsResp {
    fn from(ss_skill_info: rc::SsSkillInfo) -> Self {
        HItemIdsResp::new(ss_skill_info.id, None)
    }
}
impl From<rc::SsImplantInfo> for HItemIdsResp {
    fn from(ss_implant_info: rc::SsImplantInfo) -> Self {
        HItemIdsResp::new(ss_implant_info.id, None)
    }
}
impl From<rc::SsBoosterInfo> for HItemIdsResp {
    fn from(ss_booster_info: rc::SsBoosterInfo) -> Self {
        HItemIdsResp::new(ss_booster_info.id, None)
    }
}
impl From<rc::SsShipInfo> for HItemIdsResp {
    fn from(ss_ship_info: rc::SsShipInfo) -> Self {
        HItemIdsResp::new(ss_ship_info.id, None)
    }
}
impl From<rc::SsStructureInfo> for HItemIdsResp {
    fn from(ss_structure_info: rc::SsStructureInfo) -> Self {
        HItemIdsResp::new(ss_structure_info.id, None)
    }
}
impl From<rc::SsStanceInfo> for HItemIdsResp {
    fn from(ss_stance_info: rc::SsStanceInfo) -> Self {
        HItemIdsResp::new(ss_stance_info.id, None)
    }
}
impl From<rc::SsSubsystemInfo> for HItemIdsResp {
    fn from(ss_subsystem_info: rc::SsSubsystemInfo) -> Self {
        HItemIdsResp::new(ss_subsystem_info.id, None)
    }
}
impl From<rc::SsModuleInfo> for HItemIdsResp {
    fn from(ss_module_info: rc::SsModuleInfo) -> Self {
        HItemIdsResp::new(ss_module_info.id, ss_module_info.ss_charge_info)
    }
}
impl From<rc::SsRigInfo> for HItemIdsResp {
    fn from(ss_rig_info: rc::SsRigInfo) -> Self {
        HItemIdsResp::new(ss_rig_info.id, None)
    }
}
impl From<rc::SsDroneInfo> for HItemIdsResp {
    fn from(ss_drone_info: rc::SsDroneInfo) -> Self {
        HItemIdsResp::new(ss_drone_info.id, None)
    }
}
impl From<rc::SsFighterInfo> for HItemIdsResp {
    fn from(ss_fighter_info: rc::SsFighterInfo) -> Self {
        HItemIdsResp::new(ss_fighter_info.id, None)
    }
}
impl From<rc::SsSwEffectInfo> for HItemIdsResp {
    fn from(ss_sw_effect_info: rc::SsSwEffectInfo) -> Self {
        HItemIdsResp::new(ss_sw_effect_info.id, None)
    }
}
impl From<rc::SsFwEffectInfo> for HItemIdsResp {
    fn from(ss_fw_effect_info: rc::SsFwEffectInfo) -> Self {
        HItemIdsResp::new(ss_fw_effect_info.id, None)
    }
}
impl From<rc::SsProjEffectInfo> for HItemIdsResp {
    fn from(ss_proj_effect_info: rc::SsProjEffectInfo) -> Self {
        HItemIdsResp::new(ss_proj_effect_info.id, None)
    }
}
