use std::collections::HashMap;

use crate::{
    defs::{ReeId, ReeInt},
    util::KeyedStorage2L,
    ReeFloat,
};

use super::support::{AffectionRegister, SsAttrVal};

pub(in crate::ss::svc) struct CalcData {
    pub(in crate::ss::svc::calc) attrs: HashMap<ReeId, HashMap<ReeInt, SsAttrVal>>,
    pub(in crate::ss::svc::calc) caps: KeyedStorage2L<ReeId, ReeInt, ReeInt>,
    pub(in crate::ss::svc::calc) affections: AffectionRegister,
}
impl CalcData {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            attrs: HashMap::new(),
            caps: KeyedStorage2L::new(),
            affections: AffectionRegister::new(),
        }
    }
}
