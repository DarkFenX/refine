use serde::Deserialize;

use super::shared::HStatTimeOptions;
use crate::util::default_true;

#[derive(Copy, Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatOptionFitMining {
    #[serde(default)]
    pub(in crate::cmd) item_kinds: HStatMiningItemKinds,
    #[serde(default)]
    pub(in crate::cmd) time_options: HStatTimeOptions,
}

#[derive(Copy, Clone, educe::Educe, Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemMining {
    #[serde(default)]
    pub(in crate::cmd) time_options: HStatTimeOptions,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
}

#[derive(Copy, Clone, educe::Educe, Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatMiningItemKinds {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    module: Option<bool>,
    minion: Option<bool>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatMiningItemKinds {
    pub(in crate::cmd::stats) fn into_core(self) -> rc::stats::StatMiningItemKinds {
        let mut core_item_kinds = match self.default {
            true => rc::stats::StatMiningItemKinds::all_enabled(),
            false => rc::stats::StatMiningItemKinds::all_disabled(),
        };
        if let Some(modules) = self.module {
            core_item_kinds.module = modules;
        }
        if let Some(minions) = self.minion {
            core_item_kinds.minion = minions;
        }
        core_item_kinds
    }
}
