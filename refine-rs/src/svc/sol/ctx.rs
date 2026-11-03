use crate::{SolId, SrcAlias};

/// Carries data not available on core sol, but sometimes is needed for command execution.
#[derive(Copy, Clone)]
pub(crate) struct SolCtx {
    pub(crate) sol_id: SolId,
    pub(crate) src_alias: SrcAlias,
}
