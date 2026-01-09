use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::util::default_one;

#[derive(Clone, educe::Educe, Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionCapSim {
    #[serde(default = "default_one")]
    #[educe(Default = 1)]
    pub(in crate::cmd) cap_perc: f64,
    #[serde(default)]
    pub(in crate::cmd) reload_optionals: Option<bool>,
    #[serde(default)]
    pub(in crate::cmd) stagger: HStatOptionCapSimStagger,
}

#[serde_as]
#[derive(Clone, educe::Educe, Deserialize)]
#[educe(Default)]
#[serde(untagged)]
pub(in crate::cmd) enum HStatOptionCapSimStagger {
    #[educe(Default)]
    Simple(bool),
    Extended(bool, #[serde_as(as = "Vec<DisplayFromStr>")] Vec<rc::ItemId>),
}
impl From<&HStatOptionCapSimStagger> for rc::stats::StatCapSimStagger {
    fn from(h_stagger: &HStatOptionCapSimStagger) -> Self {
        match h_stagger {
            HStatOptionCapSimStagger::Simple(default) => rc::stats::StatCapSimStagger::new(*default),
            HStatOptionCapSimStagger::Extended(default, exceptions) => {
                let mut core_stagger = rc::stats::StatCapSimStagger::new(*default);
                core_stagger.exception_item_ids.extend(exceptions);
                core_stagger
            }
        }
    }
}
