use std::sync::Arc;

pub(crate) use booster::Booster;
pub(crate) use character::Character;
pub(crate) use implant::Implant;
pub(crate) use rig::Rig;
pub(crate) use ship::Ship;
pub(crate) use skill::Skill;
pub(crate) use stance::Stance;
pub(crate) use subsystem::Subsystem;

use crate::{consts::State, ReeId, ReeInt, Src};

mod booster;
mod character;
mod implant;
mod rig;
mod ship;
mod skill;
mod stance;
mod subsystem;

pub(crate) enum Item {
    Booster(Booster),
    Character(Character),
    Implant(Implant),
    Rig(Rig),
    Ship(Ship),
    Skill(Skill),
    Stance(Stance),
    Subsystem(Subsystem),
}
impl Item {
    pub(crate) fn get_id(&self) -> ReeId {
        match self {
            Item::Booster(i) => i.item_id,
            Item::Character(i) => i.item_id,
            Item::Implant(i) => i.item_id,
            Item::Rig(i) => i.item_id,
            Item::Ship(i) => i.item_id,
            Item::Skill(i) => i.item_id,
            Item::Stance(i) => i.item_id,
            Item::Subsystem(i) => i.item_id,
        }
    }
    pub(crate) fn get_fit_id(&self) -> ReeId {
        match self {
            Item::Booster(i) => i.fit_id,
            Item::Character(i) => i.fit_id,
            Item::Implant(i) => i.fit_id,
            Item::Rig(i) => i.fit_id,
            Item::Ship(i) => i.fit_id,
            Item::Skill(i) => i.fit_id,
            Item::Stance(i) => i.fit_id,
            Item::Subsystem(i) => i.fit_id,
        }
    }
    pub(crate) fn get_type_id(&self) -> ReeInt {
        match self {
            Item::Booster(i) => i.type_id,
            Item::Character(i) => i.type_id,
            Item::Implant(i) => i.type_id,
            Item::Rig(i) => i.type_id,
            Item::Ship(i) => i.type_id,
            Item::Skill(i) => i.type_id,
            Item::Stance(i) => i.type_id,
            Item::Subsystem(i) => i.type_id,
        }
    }
    pub(crate) fn get_state(&self) -> State {
        match self {
            Item::Booster(i) => i.state,
            Item::Character(_) => State::Offline,
            Item::Implant(i) => i.state,
            Item::Rig(_) => State::Offline,
            Item::Ship(_) => State::Offline,
            Item::Skill(_) => State::Offline,
            Item::Stance(_) => State::Offline,
            Item::Subsystem(_) => State::Offline,
        }
    }
    pub(crate) fn reload_cached_item(&mut self, src: &Arc<Src>) {
        let type_id = self.get_type_id();
        let cached_item = src.cache_handler.get_item(type_id);
        match self {
            Item::Booster(i) => i.citem = cached_item,
            Item::Character(i) => i.citem = cached_item,
            Item::Implant(i) => i.citem = cached_item,
            Item::Rig(i) => i.citem = cached_item,
            Item::Ship(i) => i.citem = cached_item,
            Item::Skill(i) => i.citem = cached_item,
            Item::Stance(i) => i.citem = cached_item,
            Item::Subsystem(i) => i.citem = cached_item,
        }
    }
}
