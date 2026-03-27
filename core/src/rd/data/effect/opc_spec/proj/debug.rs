use super::main::REffectProjOpcSpec;
use crate::{dbg::DebugResult, nd::NEffectOutputGetter, ud::UData};

impl<BG> REffectProjOpcSpec<BG>
where
    BG: NEffectOutputGetter,
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
