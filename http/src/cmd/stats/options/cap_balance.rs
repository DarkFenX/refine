use serde::Deserialize;

use super::shared::HStatTimeOptions;
use crate::util::default_true;

#[derive(Copy, Clone, Deserialize)]
pub(in crate::cmd) struct HStatOptionCapBalance {
    #[serde(default)]
    pub(in crate::cmd) src_kinds: HStatCapSrcKinds,
    #[serde(default = "default_time_options")]
    pub(in crate::cmd) time_options: HStatTimeOptions,
}
// Custom default implementation to use Sim mode instead of default Burst mode
impl Default for HStatOptionCapBalance {
    fn default() -> Self {
        Self {
            src_kinds: Default::default(),
            time_options: default_time_options(),
        }
    }
}

fn default_time_options() -> HStatTimeOptions {
    HStatTimeOptions::Sim(Default::default())
}

#[derive(Copy, Clone, educe::Educe, Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatCapSrcKinds {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    regen: Option<HStatCapRegenOptions>,
    cap_injectors: Option<bool>,
    nosfs: Option<bool>,
    consumers: Option<bool>,
    incoming_transfers: Option<bool>,
    incoming_neuts: Option<bool>,
}

#[derive(Copy, Clone, Deserialize)]
#[serde(untagged)]
enum HStatCapRegenOptions {
    Simple(bool),
    Extended(bool, HStatCapRegenOptionsFull),
}
impl HStatCapRegenOptions {
    fn is_enabled(&self) -> bool {
        match self {
            Self::Simple(enabled) => *enabled,
            Self::Extended(enabled, _) => *enabled,
        }
    }
    fn get_cap_perc(&self) -> Option<f64> {
        match self {
            Self::Simple(_) => None,
            Self::Extended(_, options) => options.cap_perc,
        }
    }
}
#[derive(Copy, Clone, Deserialize)]
struct HStatCapRegenOptionsFull {
    cap_perc: Option<f64>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatCapSrcKinds {
    pub(in crate::cmd::stats) fn into_core(self) -> rc::stats::StatCapSrcKinds {
        let mut core_src_kinds = match self.default {
            true => rc::stats::StatCapSrcKinds::all_enabled(),
            false => rc::stats::StatCapSrcKinds::all_disabled(),
        };
        if let Some(regen) = self.regen {
            core_src_kinds.regen.enabled = regen.is_enabled();
            core_src_kinds.regen.cap_perc = rc::UnitInterval::from_f64_clamped(regen.get_cap_perc().unwrap_or(0.25));
        }
        if let Some(cap_injectors) = self.cap_injectors {
            core_src_kinds.cap_injectors = cap_injectors;
        }
        if let Some(nosfs) = self.nosfs {
            core_src_kinds.nosfs = nosfs;
        }
        if let Some(consumers) = self.consumers {
            core_src_kinds.consumers = consumers;
        }
        if let Some(incoming_transfers) = self.incoming_transfers {
            core_src_kinds.incoming_transfers = incoming_transfers;
        }
        if let Some(incoming_neuts) = self.incoming_neuts {
            core_src_kinds.incoming_neuts = incoming_neuts;
        }
        core_src_kinds
    }
}
