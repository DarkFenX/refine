use crate::{ec::attrs, ed::EAttrId};

pub(crate) const BUFF_MERGE_ATTR_IDS: [EAttrId; 4] = [
    attrs::WARFARE_BUFF1_ID,
    attrs::WARFARE_BUFF2_ID,
    attrs::WARFARE_BUFF3_ID,
    attrs::WARFARE_BUFF4_ID,
];

pub(crate) const TYPE_LIST_ATTR_IDS: [EAttrId; 2] = [attrs::TGT_FILTER_TYPELIST_ID, attrs::VALID_TGT_WHITELIST];
