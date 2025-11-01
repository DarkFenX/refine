use crate::util::default_true;

#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionCapBalance {
    #[serde(default)]
    pub(in crate::cmd) src_kinds: HStatCapSrcKinds,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatCapSrcKinds {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    regen: Option<HStatCapRegenOptions>,
    cap_boosters: Option<bool>,
    consumers: Option<HStatCapConsumerOptions>,
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
            if let Some(regen_enabled) = regen.is_enabled() {
                core_src_kinds.regen.enabled = regen_enabled;
            }
            core_src_kinds.regen.cap_perc = regen.get_cap_perc();
        }
        if let Some(cap_boosters) = h_src_kinds.cap_boosters {
            core_src_kinds.cap_boosters = cap_boosters;
        }
        if let Some(consumers) = h_src_kinds.consumers {
            if let Some(consumers_enabled) = consumers.is_enabled() {
                core_src_kinds.consumers.enabled = consumers_enabled;
            }
            if let Some(reload) = consumers.is_reload() {
                core_src_kinds.consumers.reload = reload;
            }
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
    Full(HStatCapRegenOptionsFull),
}
impl HStatCapRegenOptions {
    fn is_enabled(&self) -> Option<bool> {
        match self {
            Self::Simple(enabled) => Some(*enabled),
            Self::Full(options) => options.enabled,
        }
    }
    fn get_cap_perc(&self) -> Option<rc::AttrVal> {
        match self {
            Self::Simple(_) => None,
            Self::Full(options) => options.cap_perc,
        }
    }
}
#[derive(Copy, Clone, serde::Deserialize)]
struct HStatCapRegenOptionsFull {
    enabled: Option<bool>,
    cap_perc: Option<rc::AttrVal>,
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(untagged)]
enum HStatCapConsumerOptions {
    Simple(bool),
    Full(HStatCapConsumerOptionsFull),
}
impl HStatCapConsumerOptions {
    fn is_enabled(&self) -> Option<bool> {
        match self {
            Self::Simple(enabled) => Some(*enabled),
            Self::Full(options) => options.enabled,
        }
    }
    fn is_reload(&self) -> Option<bool> {
        match self {
            Self::Simple(_) => None,
            Self::Full(options) => options.reload,
        }
    }
}
#[derive(Copy, Clone, serde::Deserialize)]
struct HStatCapConsumerOptionsFull {
    enabled: Option<bool>,
    reload: Option<bool>,
}
