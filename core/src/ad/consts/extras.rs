use ordered_float::OrderedFloat as OF;

use crate::ad::{AAttrId, AAttrVal, consts::attrs};
pub(crate) use crate::ed::consts::extras::BUFF_STDATTR_IDS;

pub(crate) const MAX_SUBCAP_MODULE_VOLUME: AAttrVal = OF(3500.0);

pub(crate) const BUFF_STDATTRS: [(AAttrId, AAttrId); 4] = [
    (attrs::WARFARE_BUFF1_ID, attrs::WARFARE_BUFF1_VAL),
    (attrs::WARFARE_BUFF2_ID, attrs::WARFARE_BUFF2_VAL),
    (attrs::WARFARE_BUFF3_ID, attrs::WARFARE_BUFF3_VAL),
    (attrs::WARFARE_BUFF4_ID, attrs::WARFARE_BUFF4_VAL),
];
