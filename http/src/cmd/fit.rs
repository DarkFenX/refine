use crate::cmd::{
    shared::{AddMode, State},
    ss,
};

pub(crate) trait FillFitCmd<T> {
    fn fill_fit(self, fit_id: reefast::ReeId) -> T;
}

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum FitCommand {
    SetShip(SetShipCmd),
    AddModuleHigh(AddModuleCmd),
    AddModuleMid(AddModuleCmd),
    AddModuleLow(AddModuleCmd),
}
impl FillFitCmd<ss::SsCommand> for FitCommand {
    fn fill_fit(self, fit_id: reefast::ReeId) -> ss::SsCommand {
        match self {
            FitCommand::SetShip(cmd) => ss::SsCommand::SetShip(cmd.fill_fit(fit_id)),
            FitCommand::AddModuleHigh(cmd) => ss::SsCommand::AddModuleHigh(cmd.fill_fit(fit_id)),
            FitCommand::AddModuleMid(cmd) => ss::SsCommand::AddModuleMid(cmd.fill_fit(fit_id)),
            FitCommand::AddModuleLow(cmd) => ss::SsCommand::AddModuleLow(cmd.fill_fit(fit_id)),
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct SetShipCmd {
    pub(crate) ship_type_id: reefast::ReeInt,
    pub(crate) state: Option<bool>,
}
impl FillFitCmd<ss::SetShipCmd> for SetShipCmd {
    fn fill_fit(self, fit_id: reefast::ReeId) -> ss::SetShipCmd {
        ss::SetShipCmd::new(fit_id, self.ship_type_id, self.state)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct AddModuleCmd {
    pub(crate) add_mode: AddMode,
    pub(crate) module_type_id: reefast::ReeInt,
    pub(crate) charge_type_id: Option<reefast::ReeInt>,
    pub(crate) state: State,
}
impl FillFitCmd<ss::AddModuleCmd> for AddModuleCmd {
    fn fill_fit(self, fit_id: reefast::ReeId) -> ss::AddModuleCmd {
        ss::AddModuleCmd::new(
            fit_id,
            self.add_mode,
            self.module_type_id,
            self.charge_type_id,
            self.state,
        )
    }
}
