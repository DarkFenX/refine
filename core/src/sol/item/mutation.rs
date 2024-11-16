use crate::{
    ad,
    defs::{AttrVal, EAttrId, EMutaId, MutaRange},
    util::StMap,
    EItemId,
};

#[derive(Clone)]
pub(in crate::sol) struct SolItemMutation {
    // Following fields are part of item skeleton
    mutaplasmid_id: EMutaId,
    mutations: StMap<EAttrId, MutaRange>,
    // Following fields are stored for fast access / optimization
    a_mutated_item: Option<ad::ArcItem>,
    merged_attrs: StMap<EAttrId, AttrVal>,
}
impl SolItemMutation {
    pub(in crate::sol::item) fn new(mutaplasmid_id: EMutaId) -> Self {
        Self {
            mutaplasmid_id,
            mutations: StMap::new(),
            a_mutated_item: None,
            merged_attrs: StMap::new(),
        }
    }
    pub(in crate::sol::item) fn get_item_type_id(&self) -> Option<EItemId> {
        self.a_mutated_item.as_ref().map(|v| v.id)
    }
}
