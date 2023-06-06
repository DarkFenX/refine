#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum CmdResp {
    NoData,
    ItemIds(ItemIdsResp),
}

#[derive(serde::Serialize)]
pub(crate) struct ItemIdsResp {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_id: Option<String>,
}
impl ItemIdsResp {
    fn new(item_id: rc::ReeId, charge_info: Option<rc::SsChargeInfo>) -> Self {
        Self {
            id: item_id.to_string(),
            charge_id: charge_info.map(|v| v.id.to_string()),
        }
    }
}
impl From<rc::SsImplantInfo> for ItemIdsResp {
    fn from(value: rc::SsImplantInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
impl From<rc::SsShipInfo> for ItemIdsResp {
    fn from(value: rc::SsShipInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
impl From<rc::SsModuleInfo> for ItemIdsResp {
    fn from(value: rc::SsModuleInfo) -> Self {
        ItemIdsResp::new(value.id, value.ss_charge_info)
    }
}
impl From<rc::SsRigInfo> for ItemIdsResp {
    fn from(value: rc::SsRigInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
