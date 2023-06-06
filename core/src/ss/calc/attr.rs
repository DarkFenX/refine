use crate::defs::ReeFloat;

#[derive(Copy, Clone, Debug)]
pub struct SsAttr {
    pub base: ReeFloat,
    pub dogma: ReeFloat,
    pub extra: ReeFloat,
}
impl SsAttr {
    pub(in crate::ss) fn new(base: ReeFloat, dogma: ReeFloat, extra: ReeFloat) -> Self {
        Self { base, dogma, extra }
    }
}
