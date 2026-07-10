use crate::{
    api::{AttrId, ItemTypeId},
    ud::ItemId,
};

#[derive(thiserror::Error, Debug)]
#[error("attribute {attr_id} is not mutable according to mutator {mutator_type_id} on item {item_id}")]
pub struct ItemMAttrMutatorError {
    pub item_id: ItemId,
    pub attr_id: AttrId,
    pub mutator_type_id: ItemTypeId,
}
