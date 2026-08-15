use crate::{
    AutochargeInfo, BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, FwEffectInfo, ImplantInfo,
    ModuleInfo, ProjEffectInfo, RigInfo, ServiceInfo, ShipInfo, SkillInfo, StanceInfo, SubsystemInfo, SwEffectInfo,
    info::ItemInfoModesInt,
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
    pub(crate) fn from_core(core_item: &mut rc::ItemMut, item_info_modes: &ItemInfoModesInt) -> Self {
        match core_item {
            rc::ItemMut::Autocharge(core_autocharge) => Self::from_core_autocharge(core_autocharge, item_info_modes),
            rc::ItemMut::Booster(core_booster) => Self::from_core_booster(core_booster, item_info_modes),
            rc::ItemMut::Character(core_character) => Self::from_core_character(core_character, item_info_modes),
            rc::ItemMut::Charge(core_charge) => Self::from_core_charge(core_charge, item_info_modes),
            rc::ItemMut::Drone(core_drone) => Self::from_core_drone(core_drone, item_info_modes),
            rc::ItemMut::Fighter(core_fighter) => Self::from_core_fighter(core_fighter, item_info_modes),
            rc::ItemMut::FwEffect(core_fw_effect) => Self::from_core_fw_effect(core_fw_effect, item_info_modes),
            rc::ItemMut::Implant(core_implant) => Self::from_core_implant(core_implant, item_info_modes),
            rc::ItemMut::Module(core_module) => Self::from_core_module(core_module, item_info_modes),
            rc::ItemMut::ProjEffect(core_proj_effect) => Self::from_core_proj_effect(core_proj_effect, item_info_modes),
            rc::ItemMut::Rig(core_rig) => Self::from_core_rig(core_rig, item_info_modes),
            rc::ItemMut::Service(core_service) => Self::from_core_service(core_service, item_info_modes),
            rc::ItemMut::Ship(core_ship) => Self::from_core_ship(core_ship, item_info_modes),
            rc::ItemMut::Skill(core_skill) => Self::from_core_skill(core_skill, item_info_modes),
            rc::ItemMut::Stance(core_stance) => Self::from_core_stance(core_stance, item_info_modes),
            rc::ItemMut::Subsystem(core_subsystem) => Self::from_core_subsystem(core_subsystem, item_info_modes),
            rc::ItemMut::SwEffect(core_sw_effect) => Self::from_core_sw_effect(core_sw_effect, item_info_modes),
        }
    }
    fn from_core_autocharge(core_autocharge: &mut rc::AutochargeMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Autocharge(AutochargeInfo::from_core(core_autocharge, item_info_modes))
    }
    fn from_core_booster(core_booster: &mut rc::BoosterMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Booster(BoosterInfo::from_core(core_booster, item_info_modes))
    }
    fn from_core_character(core_character: &mut rc::CharacterMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Character(CharacterInfo::from_core(core_character, item_info_modes))
    }
    fn from_core_charge(core_charge: &mut rc::ChargeMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Charge(ChargeInfo::from_core(core_charge, item_info_modes))
    }
    fn from_core_drone(core_drone: &mut rc::DroneMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Drone(DroneInfo::from_core(core_drone, item_info_modes))
    }
    fn from_core_fighter(core_fighter: &mut rc::FighterMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Fighter(FighterInfo::from_core(core_fighter, item_info_modes))
    }
    fn from_core_fw_effect(core_fw_effect: &mut rc::FwEffectMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::FwEffect(FwEffectInfo::from_core(core_fw_effect, item_info_modes))
    }
    fn from_core_implant(core_implant: &mut rc::ImplantMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Implant(ImplantInfo::from_core(core_implant, item_info_modes))
    }
    fn from_core_module(core_module: &mut rc::ModuleMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Module(ModuleInfo::from_core(core_module, item_info_modes))
    }
    fn from_core_proj_effect(core_proj_effect: &mut rc::ProjEffectMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::ProjEffect(ProjEffectInfo::from_core(core_proj_effect, item_info_modes))
    }
    fn from_core_rig(core_rig: &mut rc::RigMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Rig(RigInfo::from_core(core_rig, item_info_modes))
    }
    fn from_core_service(core_service: &mut rc::ServiceMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Service(ServiceInfo::from_core(core_service, item_info_modes))
    }
    fn from_core_ship(core_ship: &mut rc::ShipMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Ship(ShipInfo::from_core(core_ship, item_info_modes))
    }
    fn from_core_skill(core_skill: &mut rc::SkillMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Skill(SkillInfo::from_core(core_skill, item_info_modes))
    }
    fn from_core_stance(core_stance: &mut rc::StanceMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Stance(StanceInfo::from_core(core_stance, item_info_modes))
    }
    fn from_core_subsystem(core_subsystem: &mut rc::SubsystemMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::Subsystem(SubsystemInfo::from_core(core_subsystem, item_info_modes))
    }
    fn from_core_sw_effect(core_sw_effect: &mut rc::SwEffectMut, item_info_modes: &ItemInfoModesInt) -> Self {
        Self::SwEffect(SwEffectInfo::from_core(core_sw_effect, item_info_modes))
    }
}
