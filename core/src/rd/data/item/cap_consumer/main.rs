use crate::{
    num::PValue,
    rd::{REffectId, REffectLocalOpcSpec},
};

#[derive(Copy, Clone)]
pub(crate) struct RItemCapConsumer {
    pub(crate) effect_rid: REffectId,
    pub(crate) opc_spec: REffectLocalOpcSpec<PValue>,
}
