use super::RahSim;
use crate::{dbg::DebugResult, ud::UData};

impl RahSim {
    pub(in crate::svc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self {
            resonances,
            by_fit,
            sim_running,
        } = self;
        for item_uid in resonances.keys() {
            item_uid.consistency_check(u_data, true)?;
            // RAH sim should never be running during debug requests
            if *sim_running {
                return Err(Default::default());
            }
        }
        for (fit_uid, item_uids) in by_fit.iter() {
            fit_uid.consistency_check(u_data)?;
            for item_uid in item_uids {
                item_uid.consistency_check(u_data, true)?;
            }
        }
        Ok(())
    }
}
