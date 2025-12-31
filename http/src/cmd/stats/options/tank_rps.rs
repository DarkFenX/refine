use super::shared::HStatTimeOptions;
use crate::util::default_quarter;

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionRps {
    pub(in crate::cmd) time_options: HStatTimeOptions,
    #[serde(default = "default_quarter")]
    #[educe(Default = 0.25)]
    pub(in crate::cmd) shield_perc: rc::AttrVal,
}
