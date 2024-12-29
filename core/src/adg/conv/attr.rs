use ordered_float::OrderedFloat as OF;

use crate::{ad, adg::GData};

pub(in crate::adg::conv) fn conv_attrs(g_data: &GData) -> Vec<ad::AAttr> {
    g_data
        .attrs
        .iter()
        .map(|v| {
            ad::AAttr::new(
                v.id,
                !v.stackable,
                v.high_is_good,
                v.default_value.unwrap_or(OF(0.0)),
                v.min_attr_id,
                v.max_attr_id,
            )
        })
        .collect()
}
