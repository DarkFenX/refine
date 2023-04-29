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
    #[serde(skip_serializing_if = "Option::is_none")]
    autocharge_ids: Option<Vec<String>>,
}
impl From<reefast::ReeId> for ItemIdsResp {
    fn from(value: reefast::ReeId) -> Self {
        Self {
            id: value.to_string(),
            charge_id: None,
            autocharge_ids: None,
        }
    }
}
impl From<reefast::IdData> for ItemIdsResp {
    fn from(value: reefast::IdData) -> Self {
        Self {
            id: value.item_id.to_string(),
            charge_id: value.charge_id.map(|v| v.to_string()),
            autocharge_ids: value.autocharge_ids.map(|v| v.iter().map(|i| i.to_string()).collect()),
        }
    }
}
