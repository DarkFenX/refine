use calc::CalcSvc;
pub use calc::SsAttrVal;

mod calc;
mod routing;

pub(in crate::ss) struct SsSvcs {
    pub(in crate::ss) calc: CalcSvc,
}
impl SsSvcs {
    pub(in crate::ss) fn new() -> Self {
        Self { calc: CalcSvc::new() }
    }
}
