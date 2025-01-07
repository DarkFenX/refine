use crate::defs::AttrVal;

pub struct SolStatResource {
    pub used: AttrVal,
    pub output: AttrVal,
}
impl SolStatResource {
    pub(in crate::sol::svc::rest) fn new(used: AttrVal, output: AttrVal) -> Self {
        SolStatResource { used, output }
    }
}
