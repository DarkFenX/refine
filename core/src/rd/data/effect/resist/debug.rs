use super::main::REffectResist;
use crate::{dbg::DebugResult, ud::UData};

impl REffectResist {
    pub(in crate::rd) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let REffectResist::Attr(attr_rid) = self {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
