use std::collections::HashMap;

use crate::defs::{AttrId, SsItemId};

use super::support::{AffectionRegister, AttrCap, SsAttrVal};

pub(in crate::ss::svc) struct CalcData {
    pub(in crate::ss::svc::calc) attrs: HashMap<SsItemId, HashMap<AttrId, SsAttrVal>>,
    pub(in crate::ss::svc::calc) caps: AttrCap,
    pub(in crate::ss::svc::calc) affections: AffectionRegister,
}
impl CalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: HashMap::new(),
            caps: AttrCap::new(),
            affections: AffectionRegister::new(),
        }
    }
}
