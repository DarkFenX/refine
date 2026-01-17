use crate::num::PValue;

#[derive(Copy, Clone)]
pub(crate) struct Ecm {
    pub(crate) radar: PValue,
    pub(crate) magnetometric: PValue,
    pub(crate) gravimetric: PValue,
    pub(crate) ladar: PValue,
    pub(crate) duration: PValue,
}
