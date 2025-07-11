use crate::{
    cmd::HFitIdResp,
    shared::{HDpsProfile, HFitSecStatus},
};

#[derive(Default, serde::Deserialize)]
pub(crate) struct HAddFitCmd {
    sec_status: Option<HFitSecStatus>,
    rah_incoming_dps: Option<HDpsProfile>,
}
impl HAddFitCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HFitIdResp {
        let mut core_fit = core_sol.add_fit();
        if let Some(sec_status) = self.sec_status {
            let core_sec_status = rc::FitSecStatus::new_clamped(sec_status);
            core_fit.set_sec_status(core_sec_status);
        }
        if let Some(rah_incoming_dps) = self.rah_incoming_dps {
            core_fit.set_rah_incoming_dps(rah_incoming_dps.into());
        }
        core_fit.into()
    }
}
