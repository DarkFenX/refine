use crate::ad::{ABuffAggrMode, ABuffId, ABuffModifiers, AOp};

pub struct ABuff {
    pub id: ABuffId,
    pub aggr_mode: ABuffAggrMode,
    pub op: AOp,
    pub mods: ABuffModifiers,
}
