use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use super::shared_time::HStatTimeOptions;
use crate::util::default_true;

#[derive(Copy, Clone, Deserialize)]
pub(in crate::cmd) struct HStatOptionCapBlc {
    #[serde(default)]
    pub(in crate::cmd) src_kinds: HStatCapBlcSrcKinds,
    #[serde(default = "default_time_options")]
    pub(in crate::cmd) time_options: HStatTimeOptions,
}
// Custom default implementation to use Sim mode instead of default Burst mode
impl Default for HStatOptionCapBlc {
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
pub(in crate::cmd) struct HStatCapBlcSrcKinds {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    regen: Option<HStatCapBlcRegen>,
    cap_injectors: Option<bool>,
    nosfs: Option<HStatCapBlcNosfs>,
    consumers: Option<bool>,
    incoming_transfers: Option<bool>,
    incoming_neuts: Option<bool>,
}

#[derive(Copy, Clone, Deserialize)]
#[serde(untagged)]
enum HStatCapBlcRegen {
    Simple(bool),
    Extended(bool, HStatCapRegenOptionsFull),
}

#[derive(Copy, Clone, Deserialize)]
struct HStatCapRegenOptionsFull {
    cap_perc: Option<f64>,
}

#[derive(Copy, Clone, Deserialize)]
#[serde(untagged)]
enum HStatCapBlcNosfs {
    Simple(bool),
    Extended(bool, HStatCapNosfsOptionsFull),
}

#[serde_as]
#[derive(Copy, Clone, Deserialize)]
struct HStatCapNosfsOptionsFull {
    #[serde_as(as = "Option<DisplayFromStr>")]
    projectee_item_id: Option<rc::ItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatCapBlcSrcKinds {
    pub(in crate::cmd::stats) fn into_core(self) -> rc::stats::StatCapBlcSrcKinds {
        let mut core_src_kinds = match self.default {
            true => rc::stats::StatCapBlcSrcKinds::all_enabled(),
            false => rc::stats::StatCapBlcSrcKinds::all_disabled(),
        };
        if let Some(regen) = self.regen {
            core_src_kinds.regen = regen.into_core();
        }
        if let Some(cap_injectors) = self.cap_injectors {
            core_src_kinds.cap_injectors = cap_injectors;
        }
        if let Some(nosfs) = self.nosfs {
            core_src_kinds.nosfs = nosfs.into_core();
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

impl HStatCapBlcRegen {
    fn into_core(self) -> rc::stats::StatCapBlcRegen {
        match self {
            Self::Simple(enabled) => match enabled {
                true => rc::stats::StatCapBlcRegen::Enabled(rc::stats::StatCapBlcRegenOptions { .. }),
                false => rc::stats::StatCapBlcRegen::Disabled,
            },
            Self::Extended(enabled, options) => match enabled {
                true => match options.cap_perc {
                    Some(cap_perc) => rc::stats::StatCapBlcRegen::Enabled(rc::stats::StatCapBlcRegenOptions {
                        cap_perc: rc::UnitInterval::from_f64_clamped(cap_perc),
                    }),
                    None => rc::stats::StatCapBlcRegen::Enabled(rc::stats::StatCapBlcRegenOptions { .. }),
                },
                false => rc::stats::StatCapBlcRegen::Disabled,
            },
        }
    }
}

impl HStatCapBlcNosfs {
    fn into_core(self) -> rc::stats::StatCapBlcNosfs {
        match self {
            Self::Simple(enabled) => match enabled {
                true => rc::stats::StatCapBlcNosfs::Enabled(rc::stats::StatCapBlcNosfs { .. }),
                false => rc::stats::StatCapBlcNosfs::Disabled,
            },
            Self::Extended(enabled, options) => match enabled {
                true => match options.projectee_item_id {
                    Some(projectee_item_id) => rc::stats::StatCapBlcNosfs::Enabled(rc::stats::StatCapBlcNosfs {
                        projectee_item_id: Some(projectee_item_id),
                    }),
                    None => rc::stats::StatCapBlcNosfs::Enabled(rc::stats::StatCapBlcNosfs { .. }),
                },
                false => rc::stats::StatCapBlcNosfs::Disabled,
            },
        }
    }
}
