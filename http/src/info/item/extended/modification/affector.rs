use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::item::extended::modification) struct HAffector {
    #[serde_as(as = "DisplayFromStr")]
    item_id: rc::ItemId,
    #[serde_as(as = "Option<DisplayFromStr>")]
    attr_id: Option<rc::AttrId>,
}
impl HAffector {
    pub(in crate::info::item::extended::modification) fn from_core(core_affector: rc::Affector) -> Self {
        Self {
            item_id: core_affector.item_id,
            attr_id: core_affector.attr_id,
        }
    }
}
