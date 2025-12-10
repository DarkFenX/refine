use crate::{
    ad::{AAttrId, AItemId, AMuta, AMutaAttrRange},
    rd::RAttrKey,
    util::{GetId, RMap},
};

// Represents a mutator (aka mutaplasmid in EVE).
//
// A mutator controls how attributes of an item it is being applied to change.
pub(crate) struct RMuta {
    pub(crate) id: AItemId,
    pub(crate) item_map: RMap<AItemId, AItemId>,
    pub(crate) attr_mods: RMap<RAttrKey, AMutaAttrRange>,
}
impl RMuta {
    pub(in crate::rd) fn from_a_muta(a_muta: &AMuta) -> Self {
        Self {
            id: a_muta.id,
            item_map: a_muta.item_map.clone(),
            attr_mods: RMap::new(),
        }
    }
    pub(in crate::rd) fn fill_key_dependents(
        &mut self,
        a_mutas: &RMap<AItemId, AMuta>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
    ) {
        let a_muta = a_mutas.get(&self.id).unwrap();
        self.attr_mods.extend(
            a_muta.attr_mods.iter().filter_map(|(attr_id, attr_range)| {
                attr_id_key_map.get(attr_id).map(|attr_key| (*attr_key, *attr_range))
            }),
        )
    }
}
impl GetId<AItemId> for RMuta {
    fn get_id(&self) -> AItemId {
        self.id
    }
}
