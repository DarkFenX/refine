use crate::{
    ss::{item::debug, SsView},
    util::DebugResult,
};

use super::SsProjEffect;

impl SsProjEffect {
    pub(in crate::ss::item) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for tgt_item_id in self.tgts.iter_tgts() {
            debug::check_item(ss_view, tgt_item_id)?;
        }
        Ok(())
    }
}
