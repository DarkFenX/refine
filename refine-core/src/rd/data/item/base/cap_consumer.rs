use crate::{
    nd::NEffectGeneralOutputGetter,
    rd::{REffectId, REffectLocalOpcSpec},
};

#[derive(Copy, Clone)]
pub(crate) struct RItemCapConsumer {
    pub(crate) effect_rid: REffectId,
    pub(crate) opc_spec: REffectLocalOpcSpec<NEffectGeneralOutputGetter>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl RItemCapConsumer {
        pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self { effect_rid, opc_spec } = self;
            effect_rid.consistency_check(u_data)?;
            opc_spec.consistency_check(u_data)?;
            Ok(())
        }
    }
}
