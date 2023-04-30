use std::collections::HashMap;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum ItemInfo {
    Id(String),
    Detailed(ItemInfoDetailed),
}
impl ItemInfo {
    pub(crate) fn extract(core_ss: &mut reefast::SolarSystem, item_id: &reefast::ReeId, expand_items: bool) -> Self {
        match expand_items {
            true => Self::Detailed(ItemInfoDetailed::extract(core_ss, item_id)),
            false => Self::Id(item_id.to_string()),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct ItemInfoDetailed {
    pub(crate) id: String,
    pub(crate) original_attrs: HashMap<reefast::ReeInt, reefast::ReeFloat>,
    pub(crate) modified_attrs: HashMap<reefast::ReeInt, reefast::ReeFloat>,
}
impl ItemInfoDetailed {
    fn extract(core_ss: &mut reefast::SolarSystem, item_id: &reefast::ReeId) -> Self {
        Self {
            id: item_id.to_string(),
            original_attrs: HashMap::new(),
            modified_attrs: HashMap::new(),
        }
    }
}
