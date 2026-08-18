use crate::{SolarSystemId, src::SrcAlias};

// Carries data not available on core sol, but sometimes is needed for command execution.
#[derive(Copy, Clone)]
pub(crate) struct SolCtx {
    pub(crate) sol_id: SolarSystemId,
    pub(crate) src_alias: SrcAlias,
}
