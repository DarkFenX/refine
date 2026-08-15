use crate::{
    AutochargeInfo, BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, FwEffectInfo, ImplantInfo,
    ItemInfoArgs, ModuleInfo, ProjEffectInfo, RigInfo, ServiceInfo, ShipInfo, SkillInfo, StanceInfo, SubsystemInfo,
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
    pub(crate) fn from_core(core_item: &mut rc::ItemMut, info_args: ItemInfoArgs) -> Self {
        match core_item {
            rc::ItemMut::Autocharge(core_autocharge) => Self::from_core_autocharge(core_autocharge, info_args),
            rc::ItemMut::Booster(core_booster) => Self::from_core_booster(core_booster, info_args),
            rc::ItemMut::Character(core_character) => Self::from_core_character(core_character, info_args),
            rc::ItemMut::Charge(core_charge) => Self::from_core_charge(core_charge, info_args),
            rc::ItemMut::Drone(core_drone) => Self::from_core_drone(core_drone, info_args),
            rc::ItemMut::Fighter(core_fighter) => Self::from_core_fighter(core_fighter, info_args),
            rc::ItemMut::FwEffect(core_fw_effect) => Self::from_core_fw_effect(core_fw_effect, info_args),
            rc::ItemMut::Implant(core_implant) => Self::from_core_implant(core_implant, info_args),
            rc::ItemMut::Module(core_module) => Self::from_core_module(core_module, info_args),
            rc::ItemMut::ProjEffect(core_proj_effect) => Self::from_core_proj_effect(core_proj_effect, info_args),
            rc::ItemMut::Rig(core_rig) => Self::from_core_rig(core_rig, info_args),
            rc::ItemMut::Service(core_service) => Self::from_core_service(core_service, info_args),
            rc::ItemMut::Ship(core_ship) => Self::from_core_ship(core_ship, info_args),
            rc::ItemMut::Skill(core_skill) => Self::from_core_skill(core_skill, info_args),
            rc::ItemMut::Stance(core_stance) => Self::from_core_stance(core_stance, info_args),
            rc::ItemMut::Subsystem(core_subsystem) => Self::from_core_subsystem(core_subsystem, info_args),
            rc::ItemMut::SwEffect(core_sw_effect) => Self::from_core_sw_effect(core_sw_effect, info_args),
        }
    }
    fn from_core_autocharge(core_autocharge: &mut rc::AutochargeMut, info_args: ItemInfoArgs) -> Self {
        Self::Autocharge(AutochargeInfo::from_core(core_autocharge, info_args))
    }
    fn from_core_booster(core_booster: &mut rc::BoosterMut, info_args: ItemInfoArgs) -> Self {
        Self::Booster(BoosterInfo::from_core(core_booster, info_args))
    }
    fn from_core_character(core_character: &mut rc::CharacterMut, info_args: ItemInfoArgs) -> Self {
        Self::Character(CharacterInfo::from_core(core_character, info_args))
    }
    fn from_core_charge(core_charge: &mut rc::ChargeMut, info_args: ItemInfoArgs) -> Self {
        Self::Charge(ChargeInfo::from_core(core_charge, info_args))
    }
    fn from_core_drone(core_drone: &mut rc::DroneMut, info_args: ItemInfoArgs) -> Self {
        Self::Drone(DroneInfo::from_core(core_drone, info_args))
    }
    fn from_core_fighter(core_fighter: &mut rc::FighterMut, info_args: ItemInfoArgs) -> Self {
        Self::Fighter(FighterInfo::from_core(core_fighter, info_args))
    }
    fn from_core_fw_effect(core_fw_effect: &mut rc::FwEffectMut, info_args: ItemInfoArgs) -> Self {
        Self::FwEffect(FwEffectInfo::from_core(core_fw_effect, info_args))
    }
    fn from_core_implant(core_implant: &mut rc::ImplantMut, info_args: ItemInfoArgs) -> Self {
        Self::Implant(ImplantInfo::from_core(core_implant, info_args))
    }
    fn from_core_module(core_module: &mut rc::ModuleMut, info_args: ItemInfoArgs) -> Self {
        Self::Module(ModuleInfo::from_core(core_module, info_args))
    }
    fn from_core_proj_effect(core_proj_effect: &mut rc::ProjEffectMut, info_args: ItemInfoArgs) -> Self {
        Self::ProjEffect(ProjEffectInfo::from_core(core_proj_effect, info_args))
    }
    fn from_core_rig(core_rig: &mut rc::RigMut, info_args: ItemInfoArgs) -> Self {
        Self::Rig(RigInfo::from_core(core_rig, info_args))
    }
    fn from_core_service(core_service: &mut rc::ServiceMut, info_args: ItemInfoArgs) -> Self {
        Self::Service(ServiceInfo::from_core(core_service, info_args))
    }
    fn from_core_ship(core_ship: &mut rc::ShipMut, info_args: ItemInfoArgs) -> Self {
        Self::Ship(ShipInfo::from_core(core_ship, info_args))
    }
    fn from_core_skill(core_skill: &mut rc::SkillMut, info_args: ItemInfoArgs) -> Self {
        Self::Skill(SkillInfo::from_core(core_skill, info_args))
    }
    fn from_core_stance(core_stance: &mut rc::StanceMut, info_args: ItemInfoArgs) -> Self {
        Self::Stance(StanceInfo::from_core(core_stance, info_args))
    }
    fn from_core_subsystem(core_subsystem: &mut rc::SubsystemMut, info_args: ItemInfoArgs) -> Self {
        Self::Subsystem(SubsystemInfo::from_core(core_subsystem, info_args))
    }
    fn from_core_sw_effect(core_sw_effect: &mut rc::SwEffectMut, info_args: ItemInfoArgs) -> Self {
        Self::SwEffect(SwEffectInfo::from_core(core_sw_effect, info_args))
    }
}
