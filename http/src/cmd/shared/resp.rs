#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum HCmdResp {
    NoData,
    ItemIds(HItemIdsResp),
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
