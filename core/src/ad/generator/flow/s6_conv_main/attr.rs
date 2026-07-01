use crate::ad::{AAttr, AAttrId, AAttrs, ADataGenerator, AValue};

impl ADataGenerator {
    pub(super) fn conv_attrs(&mut self) {
        let a_attrs = self
            .e_data
            .attrs
            .data
            .iter()
            .map(|e_attr| {
                let a_attr = AAttr {
                    id: AAttrId::from_eid(e_attr.id),
                    penalizable: !e_attr.stackable,
                    hig: e_attr.high_is_good,
                    def_val: AValue::from_efloat(e_attr.default_value),
                    min_attr_id: e_attr.min_attr_id.map(AAttrId::from_eid),
                    max_attr_id: e_attr.max_attr_id.map(AAttrId::from_eid),
                };
                (a_attr.id, a_attr)
            })
            .collect();
        self.a_data.attrs = AAttrs { data: a_attrs };
    }
}
