use crate::defs::ReeFloat;

#[derive(Copy, Clone, Debug)]
pub struct AttrVal {
    pub base: ReeFloat,
    pub dogma: ReeFloat,
    pub extra: ReeFloat,
}
impl AttrVal {
    pub(in crate::ss) fn new(base: ReeFloat, dogma: ReeFloat, extra: ReeFloat) -> Self {
        Self { base, dogma, extra }
    }
}
