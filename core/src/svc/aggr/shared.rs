use crate::def::{AttrVal, OF};

pub(super) fn process_mult(mult: AttrVal) -> Option<AttrVal> {
    match mult {
        OF(1.0) => None,
        v => Some(v),
    }
}
