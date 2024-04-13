use crate::{
    cmd::change_ss,
    info::{HItemInfo, HItemInfoMode, MkItemInfo},
};

// Endpoint to add items provides no context just like solar system endpoint, so largely reuse
// commands from there
#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HAddItemCommand {
    Character(change_ss::HSetCharacterCmd),
    Skill(change_ss::HAddSkillCmd),
    Implant(change_ss::HAddImplantCmd),
    Booster(change_ss::HAddBoosterCmd),
    Ship(change_ss::HSetShipCmd),
    Structure(change_ss::HSetStructureCmd),
    Stance(change_ss::HSetStanceCmd),
    Subsystem(change_ss::HAddSubsystemCmd),
    Module(change_ss::HAddModuleCmd),
    Rig(change_ss::HAddRigCmd),
    Drone(change_ss::HAddDroneCmd),
    Fighter(change_ss::HAddFighterCmd),
    SwEffect(change_ss::HAddSwEffectCmd),
    FwEffect(change_ss::HAddFwEffectCmd),
    ProjEffect(change_ss::HAddProjEffectCmd),
}
impl HAddItemCommand {
    pub(crate) fn execute(&self, core_ss: &mut rc::SolarSystem, item_mode: HItemInfoMode) -> rc::Result<HItemInfo> {
        match self {
            Self::Character(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Skill(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Implant(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Booster(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Ship(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Structure(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Stance(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Subsystem(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Module(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Rig(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Drone(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::Fighter(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::SwEffect(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::FwEffect(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
            Self::ProjEffect(cmd) => {
                let core_info = cmd.execute(core_ss)?;
                Ok(HItemInfo::mk_info(core_ss, &core_info, item_mode))
            }
        }
    }
}
