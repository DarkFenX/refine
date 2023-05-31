use crate::ert;

use super::Data;

pub(super) fn conv_attrs(erg_data: &Data) -> Vec<ert::Attr> {
    erg_data
        .attrs
        .iter()
        .map(|v| ert::Attr::new(v.id, !v.stackable, v.high_is_good, v.default_value, v.max_attr_id))
        .collect()
}
