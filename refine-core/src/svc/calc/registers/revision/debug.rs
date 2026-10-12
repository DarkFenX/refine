use super::RevisionRegister;
use crate::{dbg::DebugResult, svc::calc::registers::RawModStorage, ud::UData};

impl RevisionRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData, rmods: &RawModStorage) -> DebugResult {
        for ckey in self.item_add_remove.keys() {
            ckey.consistency_check(u_data, rmods)?;
        }
        Ok(())
    }
}
