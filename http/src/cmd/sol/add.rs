use serde::Deserialize;

use crate::cmd::basic::HSolAddCmdFCtx;

#[derive(Default, Deserialize)]
pub(crate) struct HSolAddCmd {
    #[serde(flatten)]
    basic: HSolAddCmdFCtx,
}
impl HSolAddCmd {
    pub(crate) fn execute(&self, src: rc::Src) -> rc::SolarSystem {
        self.basic.execute(src)
    }
}
