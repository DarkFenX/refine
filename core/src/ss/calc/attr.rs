use crate::defs::ReeFloat;

#[derive(Copy, Clone, Debug)]
pub struct SsAttrVal {
    pub base: ReeFloat,
    pub dogma: ReeFloat,
    pub extra: ReeFloat,
}
impl SsAttrVal {
    pub(in crate::ss) fn new(base: ReeFloat, dogma: ReeFloat, extra: ReeFloat) -> Self {
        Self { base, dogma, extra }
    }
}
