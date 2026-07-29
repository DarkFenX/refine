use crate::{
    dbg::DebugResult,
    nd::NEffectGeneralOutputGetter,
    rd::{REffectId, REffectLocalOpcSpec},
    ud::UData,
};

#[derive(Copy, Clone)]
pub(crate) struct RItemCapConsumer {
    pub(crate) effect_rid: REffectId,
    pub(crate) opc_spec: REffectLocalOpcSpec<NEffectGeneralOutputGetter>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItemCapConsumer {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.effect_rid.consistency_check(u_data)?;
        self.opc_spec.consistency_check(u_data)?;
        Ok(())
    }
}
