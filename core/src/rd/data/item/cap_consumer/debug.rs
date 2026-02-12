use super::main::RItemCapConsumer;
use crate::{dbg::DebugResult, ud::UData};

impl RItemCapConsumer {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.effect_rid.consistency_check(u_data)?;
        self.opc_spec.consistency_check(u_data)?;
        Ok(())
    }
}
