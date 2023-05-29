use crate::cmd::{
    shared::{AddMode, State},
    ss,
};

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum FitCommand {
    AddImplant(AddImplantCmd),
    SetShip(SetShipCmd),
    AddModuleHigh(AddModuleCmd),
    AddModuleMid(AddModuleCmd),
    AddModuleLow(AddModuleCmd),
    AddRig(AddRigCmd),
}
impl FitCommand {
    pub(crate) fn fill_fit(self, fit_id: reefast::ReeId) -> ss::SsCommand {
        match self {
            FitCommand::AddImplant(cmd) => ss::SsCommand::AddImplant(cmd.fill_fit(fit_id)),
            FitCommand::SetShip(cmd) => ss::SsCommand::SetShip(cmd.fill_fit(fit_id)),
            FitCommand::AddModuleHigh(cmd) => ss::SsCommand::AddModuleHigh(cmd.fill_fit(fit_id)),
            FitCommand::AddModuleMid(cmd) => ss::SsCommand::AddModuleMid(cmd.fill_fit(fit_id)),
            FitCommand::AddModuleLow(cmd) => ss::SsCommand::AddModuleLow(cmd.fill_fit(fit_id)),
            FitCommand::AddRig(cmd) => ss::SsCommand::AddRig(cmd.fill_fit(fit_id)),
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct AddImplantCmd {
    pub(crate) type_id: reefast::ReeInt,
    pub(crate) state: Option<bool>,
}
impl AddImplantCmd {
    fn fill_fit(self, fit_id: reefast::ReeId) -> ss::AddImplantCmd {
        ss::AddImplantCmd::new(fit_id, self.type_id, self.state)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct SetShipCmd {
    pub(crate) type_id: reefast::ReeInt,
    pub(crate) state: Option<bool>,
}
impl SetShipCmd {
    fn fill_fit(self, fit_id: reefast::ReeId) -> ss::SetShipCmd {
        ss::SetShipCmd::new(fit_id, self.type_id, self.state)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct AddModuleCmd {
    pub(crate) add_mode: AddMode,
    pub(crate) module_type_id: reefast::ReeInt,
    pub(crate) charge_type_id: Option<reefast::ReeInt>,
    pub(crate) state: State,
}
impl AddModuleCmd {
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

#[derive(serde::Deserialize)]
pub(crate) struct AddRigCmd {
    pub(crate) type_id: reefast::ReeInt,
    pub(crate) state: Option<bool>,
}
impl AddRigCmd {
    fn fill_fit(self, fit_id: reefast::ReeId) -> ss::AddRigCmd {
        ss::AddRigCmd::new(fit_id, self.type_id, self.state)
    }
}
