pub(crate) use implant::Implant;
pub(crate) use ship::Ship;
pub(crate) use skill::Skill;

use crate::{ReeId, ReeInt};

mod implant;
mod ship;
mod skill;

pub(crate) enum Item {
    Implant(Implant),
    Ship(Ship),
    Skill(Skill),
}
impl Item {
    pub(crate) fn get_id(&self) -> ReeId {
        match self {
            Item::Implant(i) => i.id,
            Item::Ship(i) => i.id,
            Item::Skill(i) => i.id,
        }
    }
    pub(crate) fn get_fit_id(&self) -> ReeId {
        match self {
            Item::Implant(i) => i.fit_id,
            Item::Ship(i) => i.fit_id,
            Item::Skill(i) => i.fit_id,
        }
    }
    pub(crate) fn get_type_id(&self) -> ReeInt {
        match self {
            Item::Implant(i) => i.type_id,
            Item::Ship(i) => i.type_id,
            Item::Skill(i) => i.type_id,
        }
    }
}
