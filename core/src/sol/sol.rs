use crate::{sol::rprojs::RProjs, src::Src, svc::Svc, uad::Uad};

// Solar system glues everything together and is actual "god object" of the lib. It holds all the
// data and exposes interface to manipulate, process and fetch it.
#[derive(Clone)]
pub struct SolarSystem {
    pub(crate) uad: Uad,
    pub(crate) svc: Svc,
    pub(in crate::sol) rprojs: RProjs,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        let svcs = Svc::new(&src);
        Self {
            uad: Uad::new(src),
            svc: svcs,
            rprojs: RProjs::new(),
        }
    }
}
