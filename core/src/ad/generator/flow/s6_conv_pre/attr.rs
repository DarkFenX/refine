use crate::{
    ad::{AAttr, AAttrId},
    ed::EData,
    util::RMap,
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn conv_attrs(e_data: &EData) -> RMap<AAttrId, AAttr> {
    e_data
        .attrs
        .data
        .iter()
        .map(|v| {
            let a_attr = AAttr {
                id: v.id.into(),
                penalizable: !v.stackable,
                hig: v.high_is_good,
                def_val: v.default_value.into(),
                min_attr_id: v.min_attr_id.map(Into::into),
                max_attr_id: v.max_attr_id.map(Into::into),
            };
            (a_attr.id, a_attr)
        })
        .collect()
}
