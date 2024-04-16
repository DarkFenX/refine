use crate::{
    ss::{
        svc::debug::{check_attr, check_item},
        SsView,
    },
    util::DebugResult,
};

use super::AttrValData;

impl AttrValData {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for (item_id, attr_map) in self.data.iter() {
            check_item(ss_view, item_id)?;
            // All attributes are supposed to be available too
            for attr_id in attr_map.keys() {
                check_attr(ss_view, attr_id)?;
            }
        }
        Ok(())
    }
}
