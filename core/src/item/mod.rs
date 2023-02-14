use std::sync::Arc;

pub(crate) use implant::Implant;
pub(crate) use ship::Ship;
pub(crate) use skill::Skill;

use crate::{ReeId, ReeInt, Src};

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
            Item::Implant(i) => i.item_id,
            Item::Ship(i) => i.item_id,
            Item::Skill(i) => i.item_id,
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
    pub(crate) fn reload_cached_item(&mut self, src: Arc<Src>) {
        let type_id = self.get_type_id();
        let cached_item = src.cache_handler.get_item(type_id);
        match self {
            Item::Implant(i) => i.citem = cached_item,
            Item::Ship(i) => i.citem = cached_item,
            Item::Skill(i) => i.citem = cached_item,
        }
    }
}
