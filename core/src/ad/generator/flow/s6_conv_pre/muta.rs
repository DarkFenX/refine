use crate::{
    ad::{AAttrId, AItemId, AMuta, AMutaAttr, AMutaAttrRange, AMutaItemConv, AMutas, AValue},
    ed::EData,
    util::RMap,
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn conv_mutas(e_data: &EData) -> AMutas {
    let mut a_mutas = RMap::new();
    for e_muta in e_data.muta_items.data.iter() {
        let a_muta = a_mutas
            .entry(AItemId::from_eid(e_muta.muta_id))
            .or_insert_with(|| AMuta {
                id: AItemId::from_eid(e_muta.muta_id),
                ..
            });
        a_muta.item_map.insert(AMutaItemConv {
            base_item_id: AItemId::from_eid(e_muta.in_item_id),
            mutated_item_id: AItemId::from_eid(e_muta.out_item_id),
        });
    }
    for e_attr_data in e_data.muta_attrs.data.iter() {
        // We are interested in attribute modifiers only for mutators which have in-out item
        // definitions
        if let Some(a_muta) = a_mutas.get_mut(&AItemId::from_eid(e_attr_data.muta_id)) {
            a_muta.attr_mods.insert(AMutaAttr {
                attr_id: AAttrId::from_eid(e_attr_data.attr_id),
                range: AMutaAttrRange {
                    min_mult: AValue::from_efloat(e_attr_data.min_attr_mult),
                    max_mult: AValue::from_efloat(e_attr_data.max_attr_mult),
                },
            });
        }
    }
    AMutas { data: a_mutas }
}
