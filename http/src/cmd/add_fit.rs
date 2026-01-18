use serde::Deserialize;

use crate::{cmd::HFitIdResp, shared::HDpsProfile};

#[derive(Default, Deserialize)]
pub(crate) struct HAddFitCmd {
    sec_status: Option<f64>,
    rah_incoming_dps: Option<HDpsProfile>,
}
impl HAddFitCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HFitIdResp {
        let mut core_fit = core_sol.add_fit();
        if let Some(sec_status) = self.sec_status {
            let core_sec_status = rc::FitSecStatus::from_f64_clamped(sec_status);
            core_fit.set_sec_status(core_sec_status);
        }
        if let Some(rah_incoming_dps) = self.rah_incoming_dps {
            core_fit.set_rah_incoming_dps(rah_incoming_dps.into_core());
        }
        core_fit.into()
    }
}
