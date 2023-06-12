use calc::CalcData;
pub use calc::SsAttrVal;

mod calc;
mod routing;

pub(in crate::ss) struct SsSvcs {
    pub(in crate::ss::svc) calc_data: CalcData,
}
impl SsSvcs {
    pub(in crate::ss) fn new() -> Self {
        Self {
            calc_data: CalcData::new(),
        }
    }
}
