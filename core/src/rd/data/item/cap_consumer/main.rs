use crate::{
    nd::NGeneralOutputGetter,
    rd::{REffectId, REffectLocalOpcSpec},
};

#[derive(Copy, Clone)]
pub(crate) struct RItemCapConsumer {
    pub(crate) effect_rid: REffectId,
    pub(crate) opc_spec: REffectLocalOpcSpec<NGeneralOutputGetter>,
}
