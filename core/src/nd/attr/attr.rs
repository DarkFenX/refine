use crate::ad::{AAttr, AAttrId};

pub(crate) type NAttrMaker = fn() -> AAttr;

pub(crate) struct NAttr {
    // Adapted data attribute ID
    pub(crate) aid: AAttrId,
    // Fields related to adapted data generation
    pub(crate) adg_make_attr_fn: Option<NAttrMaker> = None,
}
