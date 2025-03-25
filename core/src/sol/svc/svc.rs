use crate::src::Src;

use super::{calc::Calc, running_effects::RunningEffects, vast::Vast};

// TODO: add item, remove item, add projection and remove projection methods are not called in
// situations where type ID of an item changes (e.g. item mutation / unmutation, source switch with
// mutation assigned). When there are users of underlying notification methods, should review them,
// and if there are any which rely on item type ID, should call those in situations where type ID
// can potentially change
#[derive(Clone)]
pub(in crate::sol) struct Svc {
    pub(in crate::sol) calc: Calc,
    pub(in crate::sol) vast: Vast,
    pub(in crate::sol::svc) running_effects: RunningEffects,
}
impl Svc {
    pub(in crate::sol) fn new(src: &Src) -> Self {
        Self {
            calc: Calc::new(src),
            vast: Vast::new(),
            running_effects: RunningEffects::new(),
        }
    }
}
