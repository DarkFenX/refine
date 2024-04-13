use crate::cmd::{change_ss, HCmdResp};

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
    pub(crate) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        match self {
            Self::Character(cmd) => cmd.execute(core_ss),
            Self::Skill(cmd) => cmd.execute(core_ss),
            Self::Implant(cmd) => cmd.execute(core_ss),
            Self::Booster(cmd) => cmd.execute(core_ss),
            Self::Ship(cmd) => cmd.execute(core_ss),
            Self::Structure(cmd) => cmd.execute(core_ss),
            Self::Stance(cmd) => cmd.execute(core_ss),
            Self::Subsystem(cmd) => cmd.execute(core_ss),
            Self::Module(cmd) => cmd.execute(core_ss),
            Self::Rig(cmd) => cmd.execute(core_ss),
            Self::Drone(cmd) => cmd.execute(core_ss),
            Self::Fighter(cmd) => cmd.execute(core_ss),
            Self::SwEffect(cmd) => cmd.execute(core_ss),
            Self::FwEffect(cmd) => cmd.execute(core_ss),
            Self::ProjEffect(cmd) => cmd.execute(core_ss),
        }
    }
}
impl From<HAddItemCommand> for change_ss::HChangeSsCommand {
    fn from(item_cmd: HAddItemCommand) -> Self {
        match item_cmd {
            HAddItemCommand::Character(cmd) => Self::SetCharacter(cmd),
            HAddItemCommand::Skill(cmd) => Self::AddSkill(cmd),
            HAddItemCommand::Implant(cmd) => Self::AddImplant(cmd),
            HAddItemCommand::Booster(cmd) => Self::AddBooster(cmd),
            HAddItemCommand::Ship(cmd) => Self::SetShip(cmd),
            HAddItemCommand::Structure(cmd) => Self::SetStructure(cmd),
            HAddItemCommand::Stance(cmd) => Self::SetStance(cmd),
            HAddItemCommand::Subsystem(cmd) => Self::AddSubsystem(cmd),
            HAddItemCommand::Module(cmd) => Self::AddModule(cmd),
            HAddItemCommand::Rig(cmd) => Self::AddRig(cmd),
            HAddItemCommand::Drone(cmd) => Self::AddDrone(cmd),
            HAddItemCommand::Fighter(cmd) => Self::AddFighter(cmd),
            HAddItemCommand::SwEffect(cmd) => Self::AddSwEffect(cmd),
            HAddItemCommand::FwEffect(cmd) => Self::AddFwEffect(cmd),
            HAddItemCommand::ProjEffect(cmd) => Self::AddProjEffect(cmd),
        }
    }
}
