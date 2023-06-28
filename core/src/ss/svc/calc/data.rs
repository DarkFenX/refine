use std::collections::HashMap;

use crate::defs::{AttrId, SsItemId};

use super::support::{AffectionRegister, AttrCapData, AttrValData, SsAttrVal};

pub(in crate::ss::svc) struct CalcData {
    pub(in crate::ss::svc::calc) attrs: AttrValData,
    pub(in crate::ss::svc::calc) caps: AttrCapData,
    pub(in crate::ss::svc::calc) affections: AffectionRegister,
}
impl CalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: AttrValData::new(),
            caps: AttrCapData::new(),
            affections: AffectionRegister::new(),
        }
    }
}
