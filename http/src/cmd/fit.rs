use crate::cmd::{
    shared::{HAddMode, HState},
    ss,
};

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

#[derive(serde::Deserialize)]
pub(crate) struct HAddImplantCmd {
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HAddImplantCmd {
    fn fill_fit(self, fit_id: rc::ReeId) -> ss::HAddImplantCmd {
        ss::HAddImplantCmd::new(fit_id, self.type_id, self.state)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HSetShipCmd {
    fn fill_fit(self, fit_id: rc::ReeId) -> ss::HSetShipCmd {
        ss::HSetShipCmd::new(fit_id, self.type_id, self.state)
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    pub(crate) add_mode: HAddMode,
    pub(crate) module_type_id: rc::ReeInt,
    pub(crate) charge_type_id: Option<rc::ReeInt>,
    pub(crate) state: HState,
}
impl HAddModuleCmd {
    fn fill_fit(self, fit_id: rc::ReeId) -> ss::HAddModuleCmd {
        ss::HAddModuleCmd::new(
            fit_id,
            self.add_mode,
            self.module_type_id,
            self.charge_type_id,
            self.state,
        )
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HAddRigCmd {
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HAddRigCmd {
    fn fill_fit(self, fit_id: rc::ReeId) -> ss::HAddRigCmd {
        ss::HAddRigCmd::new(fit_id, self.type_id, self.state)
    }
}
