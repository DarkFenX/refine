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
}
impl FitInfoDetailed {
    fn extract(core_ss: &mut reefast::SolarSystem, fit_id: reefast::ReeId, expand_items: bool) -> Self {
        let ship = core_ss
            .get_ship(fit_id)
            .map(|v| ItemInfo::extract(core_ss, v, expand_items));
        Self {
            id: fit_id.to_string(),
            ship,
        }
    }
}
