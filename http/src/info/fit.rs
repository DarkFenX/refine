use crate::info::ItemInfo;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum FitInfo {
    Id(String),
    Detailed(FitInfoDetailed),
}
impl FitInfo {
    pub(crate) fn extract(
        core_ss: &mut reefast::SolarSystem,
        fit_id: reefast::ReeId,
        expand_fits: bool,
        expand_items: bool,
    ) -> Self {
        match expand_fits {
            true => Self::Detailed(FitInfoDetailed::extract(core_ss, fit_id, expand_items)),
            false => Self::Id(fit_id.to_string()),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct FitInfoDetailed {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ship: Option<ItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) modules_high: Vec<ItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) modules_mid: Vec<ItemInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) modules_low: Vec<ItemInfo>,
}
impl FitInfoDetailed {
    fn extract(core_ss: &mut reefast::SolarSystem, fit_id: reefast::ReeId, expand_items: bool) -> Self {
        let ship = core_ss
            .get_fit_ship_info(&fit_id)
            .map(|v| ItemInfo::extract(core_ss, &v.item_id, expand_items));
        let modules_high = core_ss
            .get_high_module_infos(&fit_id)
            .iter()
            .map(|v| ItemInfo::extract(core_ss, &v.item_id, expand_items))
            .collect();
        let modules_mid = core_ss
            .get_mid_module_infos(&fit_id)
            .iter()
            .map(|v| ItemInfo::extract(core_ss, &v.item_id, expand_items))
            .collect();
        let modules_low = core_ss
            .get_low_module_infos(&fit_id)
            .iter()
            .map(|v| ItemInfo::extract(core_ss, &v.item_id, expand_items))
            .collect();
        Self {
            id: fit_id.to_string(),
            ship,
            modules_high,
            modules_mid,
            modules_low,
        }
    }
}
