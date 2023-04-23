#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum FitInfo {
    Id(String),
    Detailed(FitInfoDetailed),
}
impl FitInfo {
    pub(crate) fn extract(sol_sys: &mut reefast::SolarSystem, id: reefast::ReeId, expand_fits: bool) -> Self {
        match expand_fits {
            true => Self::Detailed(FitInfoDetailed::new(id.to_string())),
            false => Self::Id(id.to_string()),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct FitInfoDetailed {
    pub(crate) id: String,
}
impl FitInfoDetailed {
    fn new(id: String) -> Self {
        Self { id }
    }
}
