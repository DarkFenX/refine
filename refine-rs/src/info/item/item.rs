use crate::{
    AutochargeInfo, BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, FwEffectInfo, ImplantInfo,
    ItemInfoModes, ModuleInfo, ProjEffectInfo, RigInfo, ServiceInfo, ShipInfo, SkillInfo, StanceInfo, SubsystemInfo,
    SwEffectInfo,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[derive(Clone)]
pub enum ItemInfo {
    Autocharge(AutochargeInfo),
    Booster(BoosterInfo),
    Character(CharacterInfo),
    Charge(ChargeInfo),
    Drone(DroneInfo),
    Fighter(FighterInfo),
    FwEffect(FwEffectInfo),
    Implant(ImplantInfo),
    Module(ModuleInfo),
    ProjEffect(ProjEffectInfo),
    Rig(RigInfo),
    Service(ServiceInfo),
    Ship(ShipInfo),
    Skill(SkillInfo),
    Stance(StanceInfo),
    Subsystem(SubsystemInfo),
    SwEffect(SwEffectInfo),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemInfo {
    pub(crate) fn from_core(core_item: &mut rc::ItemMut, modes: ItemInfoModes) -> Self {
        match core_item {
            rc::ItemMut::Autocharge(core_autocharge) => Self::from_core_autocharge(core_autocharge, modes),
            rc::ItemMut::Booster(core_booster) => Self::from_core_booster(core_booster, modes),
            rc::ItemMut::Character(core_character) => Self::from_core_character(core_character, modes),
            rc::ItemMut::Charge(core_charge) => Self::from_core_charge(core_charge, modes),
            rc::ItemMut::Drone(core_drone) => Self::from_core_drone(core_drone, modes),
            rc::ItemMut::Fighter(core_fighter) => Self::from_core_fighter(core_fighter, modes),
            rc::ItemMut::FwEffect(core_fw_effect) => Self::from_core_fw_effect(core_fw_effect, modes),
            rc::ItemMut::Implant(core_implant) => Self::from_core_implant(core_implant, modes),
            rc::ItemMut::Module(core_module) => Self::from_core_module(core_module, modes),
            rc::ItemMut::ProjEffect(core_proj_effect) => Self::from_core_proj_effect(core_proj_effect, modes),
            rc::ItemMut::Rig(core_rig) => Self::from_core_rig(core_rig, modes),
            rc::ItemMut::Service(core_service) => Self::from_core_service(core_service, modes),
            rc::ItemMut::Ship(core_ship) => Self::from_core_ship(core_ship, modes),
            rc::ItemMut::Skill(core_skill) => Self::from_core_skill(core_skill, modes),
            rc::ItemMut::Stance(core_stance) => Self::from_core_stance(core_stance, modes),
            rc::ItemMut::Subsystem(core_subsystem) => Self::from_core_subsystem(core_subsystem, modes),
            rc::ItemMut::SwEffect(core_sw_effect) => Self::from_core_sw_effect(core_sw_effect, modes),
        }
    }
    fn from_core_autocharge(core_autocharge: &mut rc::AutochargeMut, modes: ItemInfoModes) -> Self {
        Self::Autocharge(AutochargeInfo::from_core(core_autocharge, modes))
    }
    fn from_core_booster(core_booster: &mut rc::BoosterMut, modes: ItemInfoModes) -> Self {
        Self::Booster(BoosterInfo::from_core(core_booster, modes))
    }
    fn from_core_character(core_character: &mut rc::CharacterMut, modes: ItemInfoModes) -> Self {
        Self::Character(CharacterInfo::from_core(core_character, modes))
    }
    fn from_core_charge(core_charge: &mut rc::ChargeMut, modes: ItemInfoModes) -> Self {
        Self::Charge(ChargeInfo::from_core(core_charge, modes))
    }
    fn from_core_drone(core_drone: &mut rc::DroneMut, modes: ItemInfoModes) -> Self {
        Self::Drone(DroneInfo::from_core(core_drone, modes))
    }
    fn from_core_fighter(core_fighter: &mut rc::FighterMut, modes: ItemInfoModes) -> Self {
        Self::Fighter(FighterInfo::from_core(core_fighter, modes))
    }
    fn from_core_fw_effect(core_fw_effect: &mut rc::FwEffectMut, modes: ItemInfoModes) -> Self {
        Self::FwEffect(FwEffectInfo::from_core(core_fw_effect, modes))
    }
    fn from_core_implant(core_implant: &mut rc::ImplantMut, modes: ItemInfoModes) -> Self {
        Self::Implant(ImplantInfo::from_core(core_implant, modes))
    }
    fn from_core_module(core_module: &mut rc::ModuleMut, modes: ItemInfoModes) -> Self {
        Self::Module(ModuleInfo::from_core(core_module, modes))
    }
    fn from_core_proj_effect(core_proj_effect: &mut rc::ProjEffectMut, modes: ItemInfoModes) -> Self {
        Self::ProjEffect(ProjEffectInfo::from_core(core_proj_effect, modes))
    }
    fn from_core_rig(core_rig: &mut rc::RigMut, modes: ItemInfoModes) -> Self {
        Self::Rig(RigInfo::from_core(core_rig, modes))
    }
    fn from_core_service(core_service: &mut rc::ServiceMut, modes: ItemInfoModes) -> Self {
        Self::Service(ServiceInfo::from_core(core_service, modes))
    }
    fn from_core_ship(core_ship: &mut rc::ShipMut, modes: ItemInfoModes) -> Self {
        Self::Ship(ShipInfo::from_core(core_ship, modes))
    }
    fn from_core_skill(core_skill: &mut rc::SkillMut, modes: ItemInfoModes) -> Self {
        Self::Skill(SkillInfo::from_core(core_skill, modes))
    }
    fn from_core_stance(core_stance: &mut rc::StanceMut, modes: ItemInfoModes) -> Self {
        Self::Stance(StanceInfo::from_core(core_stance, modes))
    }
    fn from_core_subsystem(core_subsystem: &mut rc::SubsystemMut, modes: ItemInfoModes) -> Self {
        Self::Subsystem(SubsystemInfo::from_core(core_subsystem, modes))
    }
    fn from_core_sw_effect(core_sw_effect: &mut rc::SwEffectMut, modes: ItemInfoModes) -> Self {
        Self::SwEffect(SwEffectInfo::from_core(core_sw_effect, modes))
    }
}
