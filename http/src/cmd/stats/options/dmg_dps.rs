use crate::shared::HSpool;

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionFitDps {
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) reload: bool,
    pub(in crate::cmd) spool: Option<HSpool>,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemDps {
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) reload: bool,
    pub(in crate::cmd) spool: Option<HSpool>,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
}
