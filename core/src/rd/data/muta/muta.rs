use crate::{
    ad::{AAttrId, AItemId, AMuta},
    rd::{RAttrId, RMutaAttrRange},
    util::{LibGetId, RMap},
};

// Represents a mutator (aka mutaplasmid in EVE).
//
// A mutator controls how attributes of an item it is being applied to change.
pub(crate) struct RMuta {
    pub(crate) id: AItemId,
    pub(crate) item_map: RMap<AItemId, AItemId>,
    pub(crate) attr_mods: RMap<RAttrId, RMutaAttrRange>,
}
impl RMuta {
    pub(in crate::rd) fn from_a_muta(a_muta: &AMuta) -> Self {
        Self {
            id: a_muta.id,
            item_map: a_muta.item_map.clone(),
            // Fields which depend on data not available during instantiation
            attr_mods: RMap::new(),
        }
    }
    pub(in crate::rd) fn fill_runtime(
        &mut self,
        a_mutas: &RMap<AItemId, AMuta>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) {
        let a_muta = a_mutas.get(&self.id).unwrap();
        self.attr_mods
            .extend(a_muta.attr_mods.iter().filter_map(|(attr_aid, a_attr_range)| {
                attr_aid_rid_map
                    .get(attr_aid)
                    .map(|attr_rid| (*attr_rid, RMutaAttrRange::from_a_attr_range(a_attr_range)))
            }))
    }
}
impl LibGetId<AItemId> for RMuta {
    fn lib_get_id(&self) -> AItemId {
        self.id
    }
}
