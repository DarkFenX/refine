use crate::{
    ad::{AAttrId, AItemId, AMutaAttrRange},
    util::RMap,
};

#[derive(Clone)]
pub struct AMuta {
    pub id: AItemId,
    pub item_map: RMap<AItemId, AItemId>,
    pub attr_mods: RMap<AAttrId, AMutaAttrRange>,
}
impl AMuta {
    pub(crate) fn new(id: AItemId) -> Self {
        Self {
            id,
            item_map: RMap::new(),
            attr_mods: RMap::new(),
        }
    }
}
