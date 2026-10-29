use crate::{
    misc::DmgKinds,
    svc::calc::CalcAttrVals,
    ud::{UFitId, UItemId},
    util::{RMap, SSLabRSet},
};

#[derive(Clone)]
pub(in crate::svc::calc) struct RahSim {
    pub(super) resonances: RMap<UItemId, Option<DmgKinds<CalcAttrVals>>>,
    pub(super) by_fit: SSLabRSet<UFitId, UItemId>,
    pub(super) sim_running: bool,
}
impl RahSim {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            resonances: RMap::new(),
            by_fit: SSLabRSet::new(),
            sim_running: false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
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
}
