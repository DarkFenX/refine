use crate::{cmd::stats::options::dmg_item_kind::HStatDmgItemKinds, shared::HSpool};

#[serde_with::serde_as]
#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionFitDps {
    #[serde(default)]
    pub(in crate::cmd) item_kinds: HStatDmgItemKinds,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) reload: bool,
    pub(in crate::cmd) spool: Option<HSpool>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}

#[serde_with::serde_as]
#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemDps {
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) reload: bool,
    pub(in crate::cmd) spool: Option<HSpool>,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) include_charges: bool,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}
