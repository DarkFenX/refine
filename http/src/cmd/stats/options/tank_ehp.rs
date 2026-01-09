use serde::Deserialize;

use crate::shared::HDpsProfile;

#[derive(Copy, Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatOptionEhp {
    pub(in crate::cmd) incoming_dps: Option<HDpsProfile>,
}
