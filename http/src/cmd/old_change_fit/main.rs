use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp,
        old_change_fit::{
            HAddFighterCmd, HAddFwEffectCmd, HAddModuleCmd, HAddServiceCmd, HAddSkillCmd, HAddSubsystemCmd,
            HChangeAutochargeCmd, HChangeCharacterCmd, HChangeChargeCmd, HChangeFighterCmd, HChangeFitCmd,
            HChangeFwEffectCmd, HChangeModuleCmd, HChangeServiceCmd, HChangeShipCmd, HChangeSkillCmd, HChangeStanceCmd,
            HChangeSubsystemCmd, HSetCharacterCmd, HSetShipCmd, HSetStanceCmd, HUnsetCharacterCmd, HUnsetShipCmd,
            HUnsetStanceCmd,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeFitCommand {
    // Fit
    ChangeFit(HChangeFitCmd),
    // Item - autocharge
    ChangeAutocharge(HChangeAutochargeCmd),
    // Item - character
    SetCharacter(HSetCharacterCmd),
    ChangeCharacter(HChangeCharacterCmd),
    UnsetCharacter(HUnsetCharacterCmd),
    // Item - charge
    ChangeCharge(HChangeChargeCmd),
    // Item - fighter
    AddFighter(HAddFighterCmd),
    ChangeFighter(HChangeFighterCmd),
    // Item - fit-wide effect
    AddFwEffect(HAddFwEffectCmd),
    ChangeFwEffect(HChangeFwEffectCmd),
    // Item - module
    AddModule(HAddModuleCmd),
    ChangeModule(HChangeModuleCmd),
    // Item - service
    AddService(HAddServiceCmd),
    ChangeService(HChangeServiceCmd),
    // Item - ship
    SetShip(HSetShipCmd),
    ChangeShip(HChangeShipCmd),
    UnsetShip(HUnsetShipCmd),
    // Item - skill
    AddSkill(HAddSkillCmd),
    ChangeSkill(HChangeSkillCmd),
    // Item - stance
    SetStance(HSetStanceCmd),
    ChangeStance(HChangeStanceCmd),
    UnsetStance(HUnsetStanceCmd),
    // Item - subsystem
    AddSubsystem(HAddSubsystemCmd),
    ChangeSubsystem(HChangeSubsystemCmd),
}
impl HChangeFitCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HCmdResp, HExecError> {
        match self {
            // Fit
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - character
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::UnsetCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - charge
            Self::ChangeCharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fighter
            Self::AddFighter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeFighter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeFwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - module
            Self::AddModule(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeModule(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - service
            Self::AddService(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeService(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - ship
            Self::SetShip(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeShip(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::UnsetShip(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - stance
            Self::SetStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::UnsetStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
