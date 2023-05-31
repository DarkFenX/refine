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
    fn new(item_id: rc::ReeId, charge_info: Option<rc::ChargeInfo>) -> Self {
        Self {
            id: item_id.to_string(),
            charge_id: charge_info.map(|v| v.id.to_string()),
        }
    }
}
impl From<rc::ImplantInfo> for ItemIdsResp {
    fn from(value: rc::ImplantInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
impl From<rc::ShipInfo> for ItemIdsResp {
    fn from(value: rc::ShipInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
impl From<rc::ModuleInfo> for ItemIdsResp {
    fn from(value: rc::ModuleInfo) -> Self {
        ItemIdsResp::new(value.id, value.charge_info)
    }
}
impl From<rc::RigInfo> for ItemIdsResp {
    fn from(value: rc::RigInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
