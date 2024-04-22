use crate::{
    cmd::change_sol,
    info::{HItemInfo, HItemInfoMode, MkItemInfo},
};

// Endpoint to add items provides no context just like solar system endpoint, so largely reuse
// commands from there
#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HAddItemCommand {
    Character(change_sol::HSetCharacterCmd),
    Skill(change_sol::HAddSkillCmd),
    Implant(change_sol::HAddImplantCmd),
    Booster(change_sol::HAddBoosterCmd),
    Ship(change_sol::HSetShipCmd),
    Structure(change_sol::HSetStructureCmd),
    Stance(change_sol::HSetStanceCmd),
    Subsystem(change_sol::HAddSubsystemCmd),
    Module(change_sol::HAddModuleCmd),
    Rig(change_sol::HAddRigCmd),
    Drone(change_sol::HAddDroneCmd),
    Fighter(change_sol::HAddFighterCmd),
    SwEffect(change_sol::HAddSwEffectCmd),
    FwEffect(change_sol::HAddFwEffectCmd),
    ProjEffect(change_sol::HAddProjEffectCmd),
}
impl HAddItemCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, item_mode: HItemInfoMode) -> rc::Result<HItemInfo> {
        match self {
            Self::Character(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Skill(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Implant(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Booster(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Ship(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Structure(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Stance(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Subsystem(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Module(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Rig(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Drone(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Fighter(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::SwEffect(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::FwEffect(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::ProjEffect(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
        }
    }
}
