use crate::{
    ad::{AAttr, AAttrId},
    def::OF,
    ed::EData,
    util::RMap,
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn conv_attrs(e_data: &EData) -> RMap<AAttrId, AAttr> {
    e_data
        .attrs
        .data
        .iter()
        .map(|v| {
            (
                v.id,
                AAttr {
                    id: v.id,
                    penalizable: !v.stackable,
                    hig: v.high_is_good,
                    def_val: OF(v.default_value),
                    min_attr_id: v.min_attr_id,
                    max_attr_id: v.max_attr_id,
                },
            )
        })
        .collect()
}
