use crate::{
    ss::{svc::svce_calc::debug::check_modifier, SsView},
    util::DebugResult,
};

use super::SsRevisionRegister;

impl SsRevisionRegister {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for ss_mod in self.item_add.iter() {
            check_modifier(ss_view, ss_mod)?;
        }
        for ss_mod in self.item_remove.iter() {
            check_modifier(ss_view, ss_mod)?;
        }
        Ok(())
    }
}
