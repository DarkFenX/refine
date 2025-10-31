#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionCapBalance {
    #[serde(default)]
    pub(in crate::cmd) src_kinds: HStatCapSrcKinds,
    pub(in crate::cmd) regen_perc: Option<rc::AttrVal>,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatCapSrcKinds {
    #[serde(default)]
    #[educe(Default = true)]
    default: bool,
    regen: Option<bool>,
    cap_boosters: Option<bool>,
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
            core_src_kinds.regen = regen;
        }
        if let Some(cap_boosters) = h_src_kinds.cap_boosters {
            core_src_kinds.cap_boosters = cap_boosters;
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
