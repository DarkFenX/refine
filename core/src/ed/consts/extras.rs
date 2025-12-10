use crate::{def::OF, ec::attrs, ed::EAttrId};

pub(crate) const BUFF_MERGE_ATTR_IDS: [EAttrId; 4] = [
    attrs::WARFARE_BUFF1_ID,
    attrs::WARFARE_BUFF2_ID,
    attrs::WARFARE_BUFF3_ID,
    attrs::WARFARE_BUFF4_ID,
];
pub(crate) const BUFF_MERGE_ATTRS: [(EAttrId, EAttrId); 4] = [
    (attrs::WARFARE_BUFF1_ID, attrs::WARFARE_BUFF1_VAL),
    (attrs::WARFARE_BUFF2_ID, attrs::WARFARE_BUFF2_VAL),
    (attrs::WARFARE_BUFF3_ID, attrs::WARFARE_BUFF3_VAL),
    (attrs::WARFARE_BUFF4_ID, attrs::WARFARE_BUFF4_VAL),
];

pub(crate) const LIMITED_PRECISION_ATTR_IDS: [EAttrId; 4] =
    [attrs::CPU, attrs::POWER, attrs::CPU_OUTPUT, attrs::POWER_OUTPUT];

pub(crate) const AU: OF<f64> = OF(149_597_870_700.0);
pub(crate) const MAX_SUBCAP_MODULE_VOLUME: OF<f64> = OF(3500.0);
