use super::main::EffectSpec;
use crate::{dbg::DebugResult, ud::UData};

impl EffectSpec {
    pub(crate) fn consistency_check(&self, u_data: &UData, check_item_load: bool) -> DebugResult {
        self.item_uid.consistency_check(u_data, check_item_load)?;
        self.effect_rid.consistency_check(u_data)?;
        Ok(())
    }
}
