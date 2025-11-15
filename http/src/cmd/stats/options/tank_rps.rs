use crate::{shared::HSpool, util::default_quarter};

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionRps {
    #[serde(default = "default_quarter")]
    #[educe(Default = 0.25)]
    pub(in crate::cmd) shield_perc: rc::AttrVal,
    pub(in crate::cmd) spool: Option<HSpool>,
}
