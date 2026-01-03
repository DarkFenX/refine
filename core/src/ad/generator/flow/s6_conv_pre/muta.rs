use crate::{
    ad::{AItemId, AMuta, AMutaAttrRange},
    ed::EData,
    util::RMap,
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn conv_mutas(e_data: &EData) -> RMap<AItemId, AMuta> {
    let mut a_mutas = RMap::new();
    for e_muta in e_data.muta_items.data.iter() {
        let a_muta = a_mutas
            .entry(e_muta.muta_id.into())
            .or_insert_with(|| AMuta::new(e_muta.muta_id.into()));
        a_muta
            .item_map
            .insert(e_muta.in_item_id.into(), e_muta.out_item_id.into());
    }
    for e_attr_data in e_data.muta_attrs.data.iter() {
        // We are interested in attribute modifiers only for mutators which have in-out item
        // definitions
        if let Some(a_muta) = a_mutas.get_mut(&e_attr_data.muta_id.into()) {
            a_muta.attr_mods.insert(
                e_attr_data.attr_id.into(),
                AMutaAttrRange {
                    min_mult: e_attr_data.min_attr_mult.into(),
                    max_mult: e_attr_data.max_attr_mult.into(),
                },
            );
        }
    }
    a_mutas
}
