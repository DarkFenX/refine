use crate::adt;

use super::Data;

pub(super) fn conv_attrs(erg_data: &Data) -> Vec<adt::AAttr> {
    erg_data
        .attrs
        .iter()
        .map(|v| adt::AAttr::new(v.id, !v.stackable, v.high_is_good, v.default_value, v.max_attr_id))
        .collect()
}
