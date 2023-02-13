use std::{fmt, sync::Arc};

use crate::{ct, util::Named, ReeId, ReeInt};

pub(crate) struct Implant {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) item: Option<Arc<ct::Item>>,
}
impl Implant {
    pub fn new(id: ReeId, fit_id: ReeId, type_id: ReeInt) -> Implant {
        Implant {
            id,
            fit_id,
            type_id,
            item: None,
        }
    }
}
impl Named for Implant {
    fn get_name() -> &'static str {
        "i:Implant"
    }
}
impl fmt::Display for Implant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(type_id={})", Implant::get_name(), self.type_id)
    }
}
