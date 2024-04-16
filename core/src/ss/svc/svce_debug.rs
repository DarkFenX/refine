use crate::ss::{svc::SsSvcs, SsView};

impl SsSvcs {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> bool {
        if !self.running_effects.debug_consistency_check(ss_view) {
            return false;
        }
        if !self.calc_data.debug_consistency_check(ss_view) {
            return false;
        }
        true
    }
}
