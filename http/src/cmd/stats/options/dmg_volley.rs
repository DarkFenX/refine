use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use super::shared::HStatTimeOptions;
use crate::cmd::stats::options::dmg_item_kind::HStatDmgItemKinds;

#[serde_as]
#[derive(Copy, Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatOptionFitVolley {
    #[serde(default)]
    pub(in crate::cmd) item_kinds: HStatDmgItemKinds,
    #[serde(default)]
    pub(in crate::cmd) time_options: HStatTimeOptions,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}

#[serde_as]
#[derive(Copy, Clone, educe::Educe, Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemVolley {
    #[serde(default)]
    pub(in crate::cmd) time_options: HStatTimeOptions,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) include_charges: bool,
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub(in crate::cmd) projectee_item_id: Option<rc::ItemId>,
}
