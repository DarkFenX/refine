use crate::{
    ad::{AAttr, AAttrId, AAttrs, ADataGenerator, AValue},
    ed::EAttr,
};

impl ADataGenerator {
    pub(super) fn conv_attrs(&mut self) {
        let a_attrs = self
            .e_data
            .attrs
            .data
            .iter()
            .map(|e_attr| {
                let EAttr {
                    id,
                    stackable,
                    high_is_good,
                    default_value,
                    min_attr_id,
                    max_attr_id,
                    // Only used to find references in attribute values during generation
                    unit_id: _,
                } = e_attr;
                let a_attr = AAttr {
                    id: AAttrId::from_eid(*id),
                    penalizable: !stackable,
                    hig: *high_is_good,
                    def_val: AValue::from_efloat(*default_value),
                    min_attr_id: min_attr_id.map(AAttrId::from_eid),
                    max_attr_id: max_attr_id.map(AAttrId::from_eid),
                };
                (a_attr.id, a_attr)
            })
            .collect();
        self.a_data.attrs = AAttrs { data: a_attrs };
    }
}
