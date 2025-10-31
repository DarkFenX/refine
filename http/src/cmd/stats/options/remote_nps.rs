use crate::util::default_true;

#[serde_with::serde_as]
#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionFitRemoteNps {
    #[serde(default)]
    pub(in crate::cmd) item_kinds: HStatNeutItemKinds,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}

#[serde_with::serde_as]
#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemRemoteNps {
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) include_charges: bool,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatNeutItemKinds {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    module: Option<bool>,
    minion: Option<bool>,
    bomb: Option<bool>,
}
impl From<&HStatNeutItemKinds> for rc::stats::StatNeutItemKinds {
    fn from(h_item_kinds: &HStatNeutItemKinds) -> Self {
        let mut core_item_kinds = match h_item_kinds.default {
            true => rc::stats::StatNeutItemKinds::all_enabled(),
            false => rc::stats::StatNeutItemKinds::all_disabled(),
        };
        if let Some(modules) = h_item_kinds.module {
            core_item_kinds.module = modules;
        }
        if let Some(minions) = h_item_kinds.minion {
            core_item_kinds.minion = minions;
        }
        if let Some(bomb) = h_item_kinds.bomb {
            core_item_kinds.bomb = bomb;
        }
        core_item_kinds
    }
}
