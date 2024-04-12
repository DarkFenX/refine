use crate::cmd::HCmdResp;

#[derive(serde::Deserialize)]
pub(crate) struct HCreateFitCmd {}
impl HCreateFitCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        Ok(core_ss.add_fit()?.into())
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HDeleteFitCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::SsItemId,
}
impl HDeleteFitCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        core_ss.remove_fit(&self.fit_id)?;
        Ok(HCmdResp::NoData)
    }
}
