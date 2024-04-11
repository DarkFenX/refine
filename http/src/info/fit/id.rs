use crate::util::HResult;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsFitId,
}
impl HFitInfoId {
    pub(in crate::info::fit) fn mk_info(core_ss: &rc::SolarSystem, fit_id: &rc::SsFitId) -> HResult<Self> {
        let core_fit = core_ss.get_fit_info(fit_id)?;
        let info = Self { id: core_fit.id };
        Ok(info)
    }
}
