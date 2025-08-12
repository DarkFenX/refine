use crate::shared::HSpool;

#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionFitRemoteRps {
    #[serde(default)]
    pub(in crate::cmd) item_kinds: HRemoteRpsItemKinds,
    pub(in crate::cmd) spool: Option<HSpool>,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemRemoteRps {
    pub(in crate::cmd) spool: Option<HSpool>,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HRemoteRpsItemKinds {
    #[serde(default)]
    #[educe(Default = true)]
    default: bool,
    modules: Option<bool>,
    minions: Option<bool>,
}
impl From<&HRemoteRpsItemKinds> for rc::stats::StatRemoteRpsItemKinds {
    fn from(h_item_kinds: &HRemoteRpsItemKinds) -> Self {
        let mut core_item_kinds = match h_item_kinds.default {
            true => rc::stats::StatRemoteRpsItemKinds::all_enabled(),
            false => rc::stats::StatRemoteRpsItemKinds::all_disabled(),
        };
        if let Some(modules) = h_item_kinds.modules {
            core_item_kinds.modules = modules;
        }
        if let Some(minions) = h_item_kinds.minions {
            core_item_kinds.minions = minions;
        }
        core_item_kinds
    }
}
