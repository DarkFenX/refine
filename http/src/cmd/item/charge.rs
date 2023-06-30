use crate::cmd::{
    shared::{apply_effect_modes, HEffectModeMap},
    HCmdResp,
};

#[derive(serde::Deserialize)]
pub(crate) struct HChangeChargeCmd {
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeChargeCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_ss: &mut rc::SolarSystem,
        item_id: &rc::SsItemId,
    ) -> rc::Result<HCmdResp> {
        apply_effect_modes(core_ss, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
