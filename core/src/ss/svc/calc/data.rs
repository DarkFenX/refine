use std::collections::HashMap;

use crate::defs::{ReeId, ReeInt};

use super::support::{AffectionRegister, SsAttrVal};

pub(in crate::ss::svc) struct CalcData {
    pub(in crate::ss::svc) attrs: HashMap<ReeId, HashMap<ReeInt, SsAttrVal>>,
    pub(in crate::ss::svc) affections: AffectionRegister,
}
impl CalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: HashMap::new(),
            affections: AffectionRegister::new(),
        }
    }
}
