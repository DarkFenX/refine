use crate::nd::{NEffectGeneralOutputGetter, NEffectProjOpcSpec};

pub(crate) struct NEffectNeut {
    pub(crate) kind: NEffectNeutKind,
    pub(crate) ospec: NEffectProjOpcSpec<NEffectGeneralOutputGetter>,
}

#[derive(Copy, Clone)]
pub(crate) enum NEffectNeutKind {
    Module,
    Minion,
    Bomb,
    SideEffect,
}
