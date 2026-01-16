use super::main::REffectProjOpcSpec;
use crate::{dbg::DebugResult, ud::UData};

impl<T> REffectProjOpcSpec<T>
where
    T: Copy,
{
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(resist) = &self.resist {
            resist.consistency_check(u_data)?;
        }
        if let Some(attr_rid) = &self.limit_attr_rid {
            attr_rid.consistency_check(u_data)?;
        }
        Ok(())
    }
}
