use crate::{
    defs::{EffectId, SsItemId},
    util::KeyedStorage1L,
};

use calc::CalcData;
pub use calc::SsAttrVal;

mod calc;
mod routing;

pub(in crate::ss) struct SsSvcs {
    pub(in crate::ss::svc) running_effects: KeyedStorage1L<SsItemId, EffectId>,
    pub(in crate::ss::svc) calc_data: CalcData,
}
impl SsSvcs {
    pub(in crate::ss) fn new() -> Self {
        Self {
            running_effects: KeyedStorage1L::new(),
            calc_data: CalcData::new(),
        }
    }
}
