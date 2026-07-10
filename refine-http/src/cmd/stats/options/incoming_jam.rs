use serde::Deserialize;

use super::shared_time::HStatTimeOptions;

#[derive(Copy, Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatOptionIncomingJam {
    #[serde(default)]
    pub(in crate::cmd) time_options: HStatTimeOptions,
}
