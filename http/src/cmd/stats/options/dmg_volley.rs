use crate::{cmd::stats::options::dmg_item_kind::HDmgItemKinds, shared::HSpool};

#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionFitVolley {
    #[serde(default)]
    pub(in crate::cmd) item_kinds: HDmgItemKinds,
    pub(in crate::cmd) spool: Option<HSpool>,
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemVolley {
    pub(in crate::cmd) spool: Option<HSpool>,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) include_charges: bool,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}
