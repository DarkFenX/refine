#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::valid) struct HValNotLoadedItemFail {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    item_ids: Vec<rc::ItemId>,
}
impl From<&rc::val::ValNotLoadedItemFail> for HValNotLoadedItemFail {
    fn from(core_val_fail: &rc::val::ValNotLoadedItemFail) -> Self {
        Self {
            item_ids: core_val_fail.item_ids.clone(),
        }
    }
}
