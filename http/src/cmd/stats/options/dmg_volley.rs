use crate::shared::HSpool;

#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionFitVolley {
    pub(in crate::cmd) spool: Option<HSpool>,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemVolley {
    pub(in crate::cmd) spool: Option<HSpool>,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
}
