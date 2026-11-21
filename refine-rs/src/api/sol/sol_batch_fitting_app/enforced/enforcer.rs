use crate::val::SolValCmdBr;

/// Decides if changes made by a fitting app batch are kept.
pub struct ValEnforcer<VC> {
    pub(super) val_cmd: SolValCmdBr,
    pub(super) checker: VC,
}
impl<VC> ValEnforcer<VC> {
    pub const fn new(val_cmd: SolValCmdBr, checker: VC) -> Self {
        Self { val_cmd, checker }
    }
}
