use crate::{
    sol::{proj_tracker::ProjTracker, svc::Svc, uad::Uad},
    src::Src,
};

// Solar system glues everything together and is actual "god object" of the lib. It holds all the
// data and exposes interface to manipulate, process and fetch it.
#[derive(Clone)]
pub struct SolarSystem {
    pub(in crate::sol) uad: Uad,
    pub(in crate::sol) svc: Svc,
    pub(in crate::sol) proj_tracker: ProjTracker,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        let svcs = Svc::new(&src);
        Self {
            uad: Uad::new(src),
            svc: svcs,
            proj_tracker: ProjTracker::new(),
        }
    }
}
