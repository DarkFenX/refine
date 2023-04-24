#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum SolSysInfo {
    Id(String),
    Detailed(SolSysInfoDetailed),
}
impl SolSysInfo {
    pub(crate) fn extract(
        sol_sys: &mut reefast::SolarSystem,
        ss_id: String,
        expand_solsys: bool,
        expand_fits: bool,
    ) -> Self {
        match expand_solsys {
            true => Self::Detailed(SolSysInfoDetailed::new(ss_id)),
            false => Self::Id(ss_id),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct SolSysInfoDetailed {
    pub(crate) id: String,
}
impl SolSysInfoDetailed {
    fn new(id: String) -> Self {
        Self { id }
    }
}
