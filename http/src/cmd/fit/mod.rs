pub(crate) use implant::HAddImplantCmd;
pub(crate) use module::HAddModuleCmd;
pub(crate) use rig::HAddRigCmd;
pub(crate) use ship::HSetShipCmd;

use crate::cmd::ss;

mod implant;
mod module;
mod rig;
mod ship;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HFitCommand {
    AddImplant(HAddImplantCmd),
    SetShip(HSetShipCmd),
    AddModuleHigh(HAddModuleCmd),
    AddModuleMid(HAddModuleCmd),
    AddModuleLow(HAddModuleCmd),
    AddRig(HAddRigCmd),
}
impl HFitCommand {
    pub(crate) fn fill_fit(self, fit_id: rc::ReeId) -> ss::HSsCommand {
        match self {
            HFitCommand::AddImplant(cmd) => ss::HSsCommand::AddImplant(cmd.fill_fit(fit_id)),
            HFitCommand::SetShip(cmd) => ss::HSsCommand::SetShip(cmd.fill_fit(fit_id)),
            HFitCommand::AddModuleHigh(cmd) => ss::HSsCommand::AddModuleHigh(cmd.fill_fit(fit_id)),
            HFitCommand::AddModuleMid(cmd) => ss::HSsCommand::AddModuleMid(cmd.fill_fit(fit_id)),
            HFitCommand::AddModuleLow(cmd) => ss::HSsCommand::AddModuleLow(cmd.fill_fit(fit_id)),
            HFitCommand::AddRig(cmd) => ss::HSsCommand::AddRig(cmd.fill_fit(fit_id)),
        }
    }
}
