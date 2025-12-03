use crate::{
    def::{AttrVal, OF},
    ec::attrs,
    ed::EAttrId,
};

pub(crate) const BUFF_MERGE_ATTR_IDS: [EAttrId; 4] = [
    attrs::WARFARE_BUFF1_ID,
    attrs::WARFARE_BUFF2_ID,
    attrs::WARFARE_BUFF3_ID,
    attrs::WARFARE_BUFF4_ID,
];

pub(crate) const AU: AttrVal = OF(149_597_870_700.0);
