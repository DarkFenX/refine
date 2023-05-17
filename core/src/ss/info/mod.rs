use crate::ss::{item::Item, SolarSystem};
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
    pub(in crate::ss) fn from_item(item: &Item, ss: &SolarSystem) -> Self {
        match item {
            Item::Booster(b) => ItemInfo::Booster(b.into()),
            Item::Character(c) => ItemInfo::Character(c.into()),
            Item::Charge(c) => ItemInfo::Charge(c.into()),
            Item::Drone(d) => ItemInfo::Drone(d.into()),
            Item::Fighter(f) => ItemInfo::Fighter(f.into()),
            Item::Implant(i) => ItemInfo::Implant(i.into()),
            Item::Module(m) => ItemInfo::Module(ss.make_mod_info(m)),
            Item::Rig(r) => ItemInfo::Rig(r.into()),
            Item::Ship(s) => ItemInfo::Ship(s.into()),
            Item::Skill(s) => ItemInfo::Skill(s.into()),
            Item::Stance(s) => ItemInfo::Stance(s.into()),
            Item::Subsystem(s) => ItemInfo::Subsystem(s.into()),
            Item::SwEffect(s) => ItemInfo::SwEffect(s.into()),
        }
    }
}
