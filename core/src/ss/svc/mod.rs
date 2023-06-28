use calc::CalcData;
pub use calc::SsAttrVal;
use effect::RunningEffects;

mod calc;
mod effect;
mod routing;

pub(in crate::ss) struct SsSvcs {
    pub(in crate::ss::svc) running_effects: RunningEffects,
    pub(in crate::ss::svc) calc_data: CalcData,
}
impl SsSvcs {
    pub(in crate::ss) fn new() -> Self {
        Self {
            running_effects: RunningEffects::new(),
            calc_data: CalcData::new(),
        }
    }
}
