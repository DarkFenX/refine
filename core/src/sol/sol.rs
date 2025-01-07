use crate::{
    sol::{misc::SolProjTracker, svc::SolSvc, uad::SolUad},
    src::Src,
};

// Solar system glues everything together and is actual "god object" of the lib. It holds all the
// data and exposes interface to manipulate, process and fetch it.
#[derive(Clone)]
pub struct SolarSystem {
    pub(in crate::sol) uad: SolUad,
    pub(in crate::sol) svc: SolSvc,
    pub(in crate::sol) proj_tracker: SolProjTracker,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        let svcs = SolSvc::new(&src);
        Self {
            uad: SolUad::new(src),
            svc: svcs,
            proj_tracker: SolProjTracker::new(),
        }
    }
}
