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
impl From<&HStatMiningItemKinds> for rc::stats::StatMiningItemKinds {
    fn from(h_item_kinds: &HStatMiningItemKinds) -> Self {
        let mut core_item_kinds = match h_item_kinds.default {
            true => rc::stats::StatMiningItemKinds::all_enabled(),
            false => rc::stats::StatMiningItemKinds::all_disabled(),
        };
        if let Some(modules) = h_item_kinds.module {
            core_item_kinds.module = modules;
        }
        if let Some(minions) = h_item_kinds.minion {
            core_item_kinds.minion = minions;
        }
        core_item_kinds
    }
}
