use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use super::shared::HStatTimeOptions;
use crate::util::default_true;

#[serde_as]
#[derive(Copy, Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatOptionFitOutRps {
    #[serde(default)]
    pub(in crate::cmd) item_kinds: HOutRepItemKinds,
    #[serde(default)]
    pub(in crate::cmd) time_options: HStatTimeOptions,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}

#[serde_as]
#[derive(Copy, Clone, educe::Educe, Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemOutRps {
    #[serde(default)]
    pub(in crate::cmd) time_options: HStatTimeOptions,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}

#[derive(Copy, Clone, educe::Educe, Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HOutRepItemKinds {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    module: Option<bool>,
    minion: Option<bool>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HOutRepItemKinds {
    pub(in crate::cmd::stats) fn into_core(self) -> rc::stats::StatOutRepItemKinds {
        let mut core_item_kinds = match self.default {
            true => rc::stats::StatOutRepItemKinds::all_enabled(),
            false => rc::stats::StatOutRepItemKinds::all_disabled(),
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
