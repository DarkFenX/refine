use crate::{CmdResps, info::SolInfo};

pub struct StatsCmdBuilderInput<'a> {
    pub ctl: &'a CmdResps,
    /// Always empty for simple batch.
    pub ctl_fix: &'a CmdResps,
    /// Info which is generated after all the control/enforcement steps are done.
    pub info: &'a SolInfo,
}
