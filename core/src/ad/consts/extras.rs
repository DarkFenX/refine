pub(crate) use crate::ec::extras::{AU, BUFF_MERGE_ATTRS};
use crate::{
    ad::{AAttrId, AAttrVal, consts::attrs},
    def::OF,
};

pub(crate) const LIMITED_PRECISION_ATTR_IDS: [AAttrId; 4] =
    [attrs::CPU, attrs::POWER, attrs::CPU_OUTPUT, attrs::POWER_OUTPUT];

pub(crate) const MAX_SUBCAP_MODULE_VOLUME: AAttrVal = OF(3500.0);
