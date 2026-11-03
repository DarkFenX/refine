use crate::{
    AutochargeInfo, BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, FwEffectInfo, ImplantInfo, ItemId,
    ItemInfoMode, ModuleInfo, ProjEffectInfo, RigInfo, ServiceInfo, ShipInfo, SkillInfo, StanceInfo, SubsystemInfo,
    SwEffectInfo, shared::OvrdMapLight,
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
    pub(in crate::info) fn from_core(
        core_item: &mut rc::Item,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        match core_item {
            rc::Item::Autocharge(core_autocharge) => Self::from_core_autocharge(core_autocharge, item_info_modes),
            rc::Item::Booster(core_booster) => Self::from_core_booster(core_booster, item_info_modes),
            rc::Item::Character(core_character) => Self::from_core_character(core_character, item_info_modes),
            rc::Item::Charge(core_charge) => Self::from_core_charge(core_charge, item_info_modes),
            rc::Item::Drone(core_drone) => Self::from_core_drone(core_drone, item_info_modes),
            rc::Item::Fighter(core_fighter) => Self::from_core_fighter(core_fighter, item_info_modes),
            rc::Item::FwEffect(core_fw_effect) => Self::from_core_fw_effect(core_fw_effect, item_info_modes),
            rc::Item::Implant(core_implant) => Self::from_core_implant(core_implant, item_info_modes),
            rc::Item::Module(core_module) => Self::from_core_module(core_module, item_info_modes),
            rc::Item::ProjEffect(core_proj_effect) => Self::from_core_proj_effect(core_proj_effect, item_info_modes),
            rc::Item::Rig(core_rig) => Self::from_core_rig(core_rig, item_info_modes),
            rc::Item::Service(core_service) => Self::from_core_service(core_service, item_info_modes),
            rc::Item::Ship(core_ship) => Self::from_core_ship(core_ship, item_info_modes),
            rc::Item::Skill(core_skill) => Self::from_core_skill(core_skill, item_info_modes),
            rc::Item::Stance(core_stance) => Self::from_core_stance(core_stance, item_info_modes),
            rc::Item::Subsystem(core_subsystem) => Self::from_core_subsystem(core_subsystem, item_info_modes),
            rc::Item::SwEffect(core_sw_effect) => Self::from_core_sw_effect(core_sw_effect, item_info_modes),
        }
    }
    fn from_core_autocharge(
        core_autocharge: &mut rc::Autocharge,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        Self::Autocharge(AutochargeInfo::from_core(core_autocharge, item_info_modes))
    }
    fn from_core_booster(core_booster: &mut rc::Booster, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Booster(BoosterInfo::from_core(core_booster, item_info_modes))
    }
    fn from_core_character(
        core_character: &mut rc::Character,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        Self::Character(CharacterInfo::from_core(core_character, item_info_modes))
    }
    fn from_core_charge(core_charge: &mut rc::Charge, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Charge(ChargeInfo::from_core(core_charge, item_info_modes))
    }
    fn from_core_drone(core_drone: &mut rc::Drone, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Drone(DroneInfo::from_core(core_drone, item_info_modes))
    }
    fn from_core_fighter(core_fighter: &mut rc::Fighter, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Fighter(FighterInfo::from_core(core_fighter, item_info_modes))
    }
    fn from_core_fw_effect(
        core_fw_effect: &mut rc::FwEffect,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        Self::FwEffect(FwEffectInfo::from_core(core_fw_effect, item_info_modes))
    }
    fn from_core_implant(core_implant: &mut rc::Implant, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Implant(ImplantInfo::from_core(core_implant, item_info_modes))
    }
    fn from_core_module(core_module: &mut rc::Module, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Module(ModuleInfo::from_core(core_module, item_info_modes))
    }
    fn from_core_proj_effect(
        core_proj_effect: &mut rc::ProjEffect,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        Self::ProjEffect(ProjEffectInfo::from_core(core_proj_effect, item_info_modes))
    }
    fn from_core_rig(core_rig: &mut rc::Rig, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Rig(RigInfo::from_core(core_rig, item_info_modes))
    }
    fn from_core_service(core_service: &mut rc::Service, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Service(ServiceInfo::from_core(core_service, item_info_modes))
    }
    fn from_core_ship(core_ship: &mut rc::Ship, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Ship(ShipInfo::from_core(core_ship, item_info_modes))
    }
    fn from_core_skill(core_skill: &mut rc::Skill, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Skill(SkillInfo::from_core(core_skill, item_info_modes))
    }
    fn from_core_stance(core_stance: &mut rc::Stance, item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>) -> Self {
        Self::Stance(StanceInfo::from_core(core_stance, item_info_modes))
    }
    fn from_core_subsystem(
        core_subsystem: &mut rc::Subsystem,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        Self::Subsystem(SubsystemInfo::from_core(core_subsystem, item_info_modes))
    }
    fn from_core_sw_effect(
        core_sw_effect: &mut rc::SwEffect,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        Self::SwEffect(SwEffectInfo::from_core(core_sw_effect, item_info_modes))
    }
}
