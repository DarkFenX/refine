use crate::{sol::rev_projs::RevProjs, src::Src, svc::Svc, ud::UData};

/// Holds all the user data, and exposes the interface to manipulate, process and fetch it.
///
/// Everything is accessed through a solar system: fits, fleets and items are reached by their IDs,
/// which returns a short-lived handle borrowing the solar system. Since handles borrow, only one
/// mutable handle can be held at a time.
#[derive(Clone)]
pub struct SolarSystem {
    pub(crate) u_data: UData,
    pub(crate) svc: Svc,
    pub(crate) rev_projs: RevProjs,
}
impl SolarSystem {
    pub fn new(src: &Src) -> Self {
        Self {
            u_data: UData::new(src.r_data.clone()),
            svc: Svc::new(),
            rev_projs: RevProjs::new(),
        }
    }
}
