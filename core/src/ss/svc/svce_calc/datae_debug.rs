use crate::ss::SsView;

use super::CalcData;

impl CalcData {
    pub(in crate::ss::svc) fn debug_consistency_check(&self, ss_view: &SsView) -> bool {
        if !self.attrs.debug_consistency_check(ss_view) {
            return false;
        }
        true
    }
}
