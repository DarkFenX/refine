pub use booster::BoosterInfo;
pub use character::CharacterInfo;
pub use charge::ChargeInfo;
pub use drone::DroneInfo;
pub use fighter::FighterInfo;
pub use implant::ImplantInfo;
pub use module::ModuleInfo;
pub use rig::RigInfo;
pub use ship::ShipInfo;
pub use skill::SkillInfo;
pub use stance::StanceInfo;
pub use subsystem::SubsystemInfo;
pub use sw_effect::SwEffectInfo;

use crate::{ss, ssi};

mod booster;
mod character;
mod charge;
mod drone;
mod fighter;
mod implant;
mod module;
mod rig;
mod ship;
mod skill;
mod stance;
mod subsystem;
mod sw_effect;

pub enum ItemInfo {
    Booster(BoosterInfo),
    Character(CharacterInfo),
    Charge(ChargeInfo),
    Drone(DroneInfo),
    Fighter(FighterInfo),
    Implant(ImplantInfo),
    Module(ModuleInfo),
    Rig(RigInfo),
    Ship(ShipInfo),
    Skill(SkillInfo),
    Stance(StanceInfo),
    Subsystem(SubsystemInfo),
    SwEffect(SwEffectInfo),
}
impl ItemInfo {
    pub(crate) fn from_item(item: &ssi::Item, ss: &ss::SolarSystem) -> Self {
        match item {
            ssi::Item::Booster(b) => ItemInfo::Booster(b.into()),
            ssi::Item::Character(c) => ItemInfo::Character(c.into()),
            ssi::Item::Charge(c) => ItemInfo::Charge(c.into()),
            ssi::Item::Drone(d) => ItemInfo::Drone(d.into()),
            ssi::Item::Fighter(f) => ItemInfo::Fighter(f.into()),
            ssi::Item::Implant(i) => ItemInfo::Implant(i.into()),
            ssi::Item::Module(m) => ItemInfo::Module(ss.make_mod_info(m)),
            ssi::Item::Rig(r) => ItemInfo::Rig(r.into()),
            ssi::Item::Ship(s) => ItemInfo::Ship(s.into()),
            ssi::Item::Skill(s) => ItemInfo::Skill(s.into()),
            ssi::Item::Stance(s) => ItemInfo::Stance(s.into()),
            ssi::Item::Subsystem(s) => ItemInfo::Subsystem(s.into()),
            ssi::Item::SwEffect(s) => ItemInfo::SwEffect(s.into()),
        }
    }
}
