use crate::shared::{HDpsProfile, HSpool};

#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionErps {
    pub(in crate::cmd) incoming_dps: Option<HDpsProfile>,
    pub(in crate::cmd) spool: Option<HSpool>,
}
