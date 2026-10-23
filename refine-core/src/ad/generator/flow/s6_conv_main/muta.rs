use crate::{
    ad::{AAttrId, ADataGenerator, AItemId, AMuta, AMutaAttr, AMutaAttrRange, AMutaItem, AMutas, AValue},
    ed::{EMutaAttr, EMutaItem},
    util::RMap,
};

impl ADataGenerator {
    pub(super) fn conv_mutas(&mut self) {
        let mut a_mutas = RMap::new();
        for e_muta in self.e_data.muta_items.data.iter() {
            let EMutaItem {
                muta_id,
                in_item_id,
                out_item_id,
            } = e_muta;
            let a_muta = a_mutas.entry(AItemId::from_eid(*muta_id)).or_insert_with(|| AMuta {
                id: AItemId::from_eid(*muta_id),
                ..
            });
            a_muta.items.insert(AMutaItem {
                base_item_id: AItemId::from_eid(*in_item_id),
                mutated_item_id: AItemId::from_eid(*out_item_id),
            });
        }
        for e_attr_data in self.e_data.muta_attrs.data.iter() {
            let EMutaAttr {
                muta_id,
                attr_id,
                min_attr_mult,
                max_attr_mult,
            } = e_attr_data;
            // We are interested in attribute modifiers only for mutators which have in-out item
            // definitions
            if let Some(a_muta) = a_mutas.get_mut(&AItemId::from_eid(*muta_id)) {
                a_muta.attrs.insert(AMutaAttr {
                    attr_id: AAttrId::from_eid(*attr_id),
                    range: AMutaAttrRange {
                        mult_min: AValue::from_efloat(*min_attr_mult),
                        mult_max: AValue::from_efloat(*max_attr_mult),
                    },
                });
            }
        }
        self.a_data.mutas = AMutas { data: a_mutas };
    }
}
