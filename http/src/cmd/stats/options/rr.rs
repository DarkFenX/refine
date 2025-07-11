use crate::shared::HSpool;

#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionRr {
    pub(in crate::cmd) spool: Option<HSpool>,
}
