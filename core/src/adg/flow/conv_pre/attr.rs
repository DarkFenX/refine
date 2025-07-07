use crate::{ad, def::OF, ed, util::RMap};

pub(in crate::adg::flow::conv_pre) fn conv_attrs(e_data: &ed::EData) -> RMap<ad::AAttrId, ad::AAttr> {
    e_data
        .attrs
        .data
        .iter()
        .map(|v| {
            (
                v.id,
                ad::AAttr {
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
