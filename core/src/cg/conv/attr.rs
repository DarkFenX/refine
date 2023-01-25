use crate::ct;

use super::CGData;

pub(super) fn conv_attrs(cg_data: &CGData) -> Vec<ct::Attr> {
    cg_data
        .attrs
        .iter()
        .map(|v| ct::Attr::new(v.id, !v.stackable, v.high_is_good, v.default_value, v.max_attr_id))
        .collect()
}
