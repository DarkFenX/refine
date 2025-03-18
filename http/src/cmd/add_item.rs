use crate::{
    cmd::change_sol,
    info::{HItemInfo, HItemInfoMode, MkItemInfo},
    util::HExecError,
};

// Endpoint to add items provides no context just like solar system endpoint, so largely reuse
// commands from there
#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HAddItemCommand {
    Booster(change_sol::HAddBoosterCmd),
    Character(change_sol::HSetCharacterCmd),
    Drone(change_sol::HAddDroneCmd),
    Fighter(change_sol::HAddFighterCmd),
    FwEffect(change_sol::HAddFwEffectCmd),
    Implant(change_sol::HAddImplantCmd),
    Module(change_sol::HAddModuleCmd),
    ProjEffect(change_sol::HAddProjEffectCmd),
    Rig(change_sol::HAddRigCmd),
    Service(change_sol::HAddServiceCmd),
    Ship(change_sol::HSetShipCmd),
    Skill(change_sol::HAddSkillCmd),
    Stance(change_sol::HSetStanceCmd),
    Subsystem(change_sol::HAddSubsystemCmd),
    SwEffect(change_sol::HAddSwEffectCmd),
}
impl HAddItemCommand {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_mode: HItemInfoMode,
    ) -> Result<HItemInfo, HExecError> {
        match self {
            Self::Booster(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Character(cmd) => {
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
            Self::FwEffect(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Implant(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Module(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::ProjEffect(cmd) => {
                let core_info = cmd.execute(core_sol);
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Rig(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Service(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Ship(cmd) => {
                let core_info = cmd.execute(core_sol)?;
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
            Self::Skill(cmd) => {
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
            Self::SwEffect(cmd) => {
                let core_info = cmd.execute(core_sol);
                Ok(HItemInfo::mk_info(core_sol, &core_info, item_mode))
            }
        }
    }
}
