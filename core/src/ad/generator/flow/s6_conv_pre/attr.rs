use crate::{
    ad::{AAttr, AAttrId, AAttrs, AValue},
    ed::EData,
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn conv_attrs(e_data: &EData) -> AAttrs {
    let a_attrs = e_data
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
    AAttrs { data: a_attrs }
}
