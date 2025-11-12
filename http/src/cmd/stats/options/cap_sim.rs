use crate::util::default_one;

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionCapSim {
    #[serde(default = "default_one")]
    #[educe(Default = 1)]
    pub(in crate::cmd) cap_perc: rc::AttrVal,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) stagger: bool,
}
