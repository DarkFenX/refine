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
    fn new(item_id: reefast_core::ReeId, charge_info: Option<reefast_core::ChargeInfo>) -> Self {
        Self {
            id: item_id.to_string(),
            charge_id: charge_info.map(|v| v.id.to_string()),
        }
    }
}
impl From<reefast_core::ImplantInfo> for ItemIdsResp {
    fn from(value: reefast_core::ImplantInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
impl From<reefast_core::ShipInfo> for ItemIdsResp {
    fn from(value: reefast_core::ShipInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
impl From<reefast_core::ModuleInfo> for ItemIdsResp {
    fn from(value: reefast_core::ModuleInfo) -> Self {
        ItemIdsResp::new(value.id, value.charge_info)
    }
}
impl From<reefast_core::RigInfo> for ItemIdsResp {
    fn from(value: reefast_core::RigInfo) -> Self {
        ItemIdsResp::new(value.id, None)
    }
}
