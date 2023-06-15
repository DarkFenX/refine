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
    fn new(item_id: rc::ReeId, charge_info: Option<rc::SsChargeInfo>) -> Self {
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
impl From<rc::SsImplantInfo> for HItemIdsResp {
    fn from(ss_implant_info: rc::SsImplantInfo) -> Self {
        HItemIdsResp::new(ss_implant_info.id, None)
    }
}
impl From<rc::SsShipInfo> for HItemIdsResp {
    fn from(ss_ship_info: rc::SsShipInfo) -> Self {
        HItemIdsResp::new(ss_ship_info.id, None)
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
