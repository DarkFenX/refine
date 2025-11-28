use crate::{shared::HSpool, util::default_true};

#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionFitOutRps {
    #[serde(default)]
    pub(in crate::cmd) item_kinds: HOutRepItemKinds,
    pub(in crate::cmd) spool: Option<HSpool>,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemOutRps {
    pub(in crate::cmd) spool: Option<HSpool>,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HOutRepItemKinds {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    module: Option<bool>,
    minion: Option<bool>,
}
impl From<&HOutRepItemKinds> for rc::stats::StatOutRepItemKinds {
    fn from(h_item_kinds: &HOutRepItemKinds) -> Self {
        let mut core_item_kinds = match h_item_kinds.default {
            true => rc::stats::StatOutRepItemKinds::all_enabled(),
            false => rc::stats::StatOutRepItemKinds::all_disabled(),
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
