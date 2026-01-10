use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        change_item::{
            HChangeAutochargeCmd, HChangeBoosterCmd, HChangeCharacterCmd, HChangeChargeCmd, HChangeDroneCmd,
            HChangeFighterCmd, HChangeFwEffectCmd, HChangeImplantCmd, HChangeModuleCmd, HChangeProjEffectCmd,
            HChangeRigCmd, HChangeServiceCmd, HChangeShipCmd, HChangeSkillCmd, HChangeStanceCmd, HChangeSubsystemCmd,
            HChangeSwEffectCmd,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeItemCommand {
    Autocharge(HChangeAutochargeCmd),
    Booster(HChangeBoosterCmd),
    Character(HChangeCharacterCmd),
    Charge(HChangeChargeCmd),
    Drone(HChangeDroneCmd),
    Fighter(HChangeFighterCmd),
    FwEffect(HChangeFwEffectCmd),
    Implant(HChangeImplantCmd),
    Module(HChangeModuleCmd),
    ProjEffect(HChangeProjEffectCmd),
    Rig(HChangeRigCmd),
    Service(HChangeServiceCmd),
    Ship(HChangeShipCmd),
    Skill(HChangeSkillCmd),
    Stance(HChangeStanceCmd),
    Subsystem(HChangeSubsystemCmd),
    SwEffect(HChangeSwEffectCmd),
}
impl HChangeItemCommand {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Autocharge(cmd) => cmd.execute(core_sol, item_id),
            Self::Booster(cmd) => cmd.execute(core_sol, item_id),
            Self::Character(cmd) => cmd.execute(core_sol, item_id),
            Self::Charge(cmd) => cmd.execute(core_sol, item_id),
            Self::Drone(cmd) => cmd.execute(core_sol, item_id),
            Self::Fighter(cmd) => cmd.execute(core_sol, item_id),
            Self::FwEffect(cmd) => cmd.execute(core_sol, item_id),
            Self::Implant(cmd) => cmd.execute(core_sol, item_id),
            Self::Module(cmd) => cmd.execute(core_sol, item_id),
            Self::ProjEffect(cmd) => cmd.execute(core_sol, item_id),
            Self::Rig(cmd) => cmd.execute(core_sol, item_id),
            Self::Service(cmd) => cmd.execute(core_sol, item_id),
            Self::Ship(cmd) => cmd.execute(core_sol, item_id),
            Self::Skill(cmd) => cmd.execute(core_sol, item_id),
            Self::Stance(cmd) => cmd.execute(core_sol, item_id),
            Self::Subsystem(cmd) => cmd.execute(core_sol, item_id),
            Self::SwEffect(cmd) => cmd.execute(core_sol, item_id),
        }
    }
}
