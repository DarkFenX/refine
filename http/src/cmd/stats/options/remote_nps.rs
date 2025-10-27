#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionFitRemoteNps {
    #[serde(default)]
    pub(in crate::cmd) item_kinds: HRemoteNpsItemKinds,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemRemoteNps {
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HRemoteNpsItemKinds {
    #[serde(default)]
    #[educe(Default = true)]
    default: bool,
    module: Option<bool>,
    minion: Option<bool>,
    bomb: Option<bool>,
}
impl From<&HRemoteNpsItemKinds> for rc::stats::StatRemoteNpsItemKinds {
    fn from(h_item_kinds: &HRemoteNpsItemKinds) -> Self {
        let mut core_item_kinds = match h_item_kinds.default {
            true => rc::stats::StatRemoteNpsItemKinds::all_enabled(),
            false => rc::stats::StatRemoteNpsItemKinds::all_disabled(),
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
