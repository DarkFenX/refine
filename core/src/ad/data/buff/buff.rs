use crate::ad::{ABuffAggrMode, ABuffId, ABuffModifier, AOp};

#[derive(Clone)]
pub struct ABuff {
    pub id: ABuffId,
    pub aggr_mode: ABuffAggrMode,
    pub op: AOp,
    pub mods: Vec<ABuffModifier>,
}
