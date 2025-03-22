use ordered_float::OrderedFloat as OF;

use crate::{ad, adg::GData};

pub(in crate::adg::flow::conv) fn conv_attrs(g_data: &GData) -> Vec<ad::AAttr> {
    g_data
        .attrs
        .iter()
        .map(|v| ad::AAttr {
            id: v.id,
            penalizable: !v.stackable,
            hig: v.high_is_good,
            def_val: OF(v.default_value),
            min_attr_id: v.min_attr_id,
            max_attr_id: v.max_attr_id,
        })
        .collect()
}
