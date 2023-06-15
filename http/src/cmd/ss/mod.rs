pub(crate) use implant::HAddImplantCmd;
pub(crate) use module::HAddModuleCmd;
pub(crate) use rig::HAddRigCmd;
pub(crate) use ship::HSetShipCmd;

mod implant;
mod module;
mod rig;
mod ship;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSsCommand {
    AddImplant(HAddImplantCmd),
    SetShip(HSetShipCmd),
    AddModuleHigh(HAddModuleCmd),
    AddModuleMid(HAddModuleCmd),
    AddModuleLow(HAddModuleCmd),
    AddRig(HAddRigCmd),
}
