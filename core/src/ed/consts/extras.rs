use crate::{ec::attrs, ed::EAttrId, def::AttrVal, def::OF};

pub(crate) const BUFF_STDATTR_IDS: [EAttrId; 4] = [
    attrs::WARFARE_BUFF1_ID,
    attrs::WARFARE_BUFF2_ID,
    attrs::WARFARE_BUFF3_ID,
    attrs::WARFARE_BUFF4_ID,
];

pub(crate) const AU: AttrVal = OF(149_597_870_700.0);
pub(crate) const LY: AttrVal = OF(9_460_000_000_000_000.0);
