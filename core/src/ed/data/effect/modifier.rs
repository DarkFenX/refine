use crate::{ed::EPrimitive, util::RMap};

pub struct EEffectMod {
    pub func: String,
    pub args: RMap<String, EPrimitive>,
}
