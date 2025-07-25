use crate::{sol::rev_projs::RevProjs, src::Src, svc::Svc, ud::UData};

// Solar system glues everything together and is actual "god object" of the lib. It holds all the
// data and exposes interface to manipulate, process and fetch it.
#[derive(Clone)]
pub struct SolarSystem {
    pub(crate) u_data: UData,
    pub(crate) svc: Svc,
    pub(in crate::sol) rev_projs: RevProjs,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        let svcs = Svc::new(&src);
        Self {
            u_data: UData::new(src),
            svc: svcs,
            rev_projs: RevProjs::new(),
        }
    }
}
