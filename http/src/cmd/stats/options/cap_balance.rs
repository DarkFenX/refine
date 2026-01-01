use super::shared::HStatTimeOptions;
use crate::util::default_true;

#[derive(Copy, Clone, serde::Deserialize)]
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

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
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
impl From<&HStatCapSrcKinds> for rc::stats::StatCapSrcKinds {
    fn from(h_src_kinds: &HStatCapSrcKinds) -> Self {
        let mut core_src_kinds = match h_src_kinds.default {
            true => rc::stats::StatCapSrcKinds::all_enabled(),
            false => rc::stats::StatCapSrcKinds::all_disabled(),
        };
        if let Some(regen) = h_src_kinds.regen {
            core_src_kinds.regen.enabled = regen.is_enabled();
            core_src_kinds.regen.cap_perc =
                rc::UnitInterval::new_clamped(regen.get_cap_perc().unwrap_or(rc::AttrVal::from(0.25)));
        }
        if let Some(cap_injectors) = h_src_kinds.cap_injectors {
            core_src_kinds.cap_injectors = cap_injectors;
        }
        if let Some(nosfs) = h_src_kinds.nosfs {
            core_src_kinds.nosfs = nosfs;
        }
        if let Some(consumers) = h_src_kinds.consumers {
            core_src_kinds.consumers = consumers;
        }
        if let Some(incoming_transfers) = h_src_kinds.incoming_transfers {
            core_src_kinds.incoming_transfers = incoming_transfers;
        }
        if let Some(incoming_neuts) = h_src_kinds.incoming_neuts {
            core_src_kinds.incoming_neuts = incoming_neuts;
        }
        core_src_kinds
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
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
    fn get_cap_perc(&self) -> Option<rc::AttrVal> {
        match self {
            Self::Simple(_) => None,
            Self::Extended(_, options) => options.cap_perc,
        }
    }
}
#[derive(Copy, Clone, serde::Deserialize)]
struct HStatCapRegenOptionsFull {
    cap_perc: Option<rc::AttrVal>,
}
