use crate::ad::{AAttrId, ADataGenerator, AItemId, AMuta, AMutaAttr, AMutaAttrRange, AMutaItem, AValue};

impl ADataGenerator {
    pub(super) fn conv_mutas(&mut self) {
        self.a_data.mutas = self
            .e_data
            .mutas
            .data
            .iter()
            .filter_map(|e_muta| match e_muta.in_item_ids.is_empty() {
                true => None,
                false => Some(AMuta {
                    id: AItemId::from_eid(e_muta.id),
                    items: e_muta
                        .in_item_ids
                        .iter()
                        .map(|&in_item_eid| AMutaItem {
                            base_item_id: AItemId::from_eid(in_item_eid),
                            mutated_item_id: AItemId::from_eid(e_muta.out_item_id),
                        })
                        .collect(),
                    attrs: e_muta
                        .attrs
                        .iter()
                        .map(|e_muta_attr| AMutaAttr {
                            attr_id: AAttrId::from_eid(e_muta_attr.attr_id),
                            range: AMutaAttrRange {
                                mult_min: AValue::from_efloat(e_muta_attr.min_attr_mult),
                                mult_max: AValue::from_efloat(e_muta_attr.max_attr_mult),
                            },
                        })
                        .collect(),
                }),
            })
            .collect();
    }
}
