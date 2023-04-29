#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub(crate) enum FitCommand {
    SetShip(SetShipCmd),
    AddModuleHigh(AddModule),
    // AddModuleMid(AddModule),
    // AddModuleLow(AddModule),
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SetShipCmd {
    pub(crate) ship_type_id: reefast::ReeInt,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct AddModule {
    pub(crate) add_mode: AddMode,
    pub(crate) module_type_id: reefast::ReeInt,
    pub(crate) state: State,
    pub(crate) charge_type_id: Option<reefast::ReeInt>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum State {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum AddMode {
    Append,
    Equip,
    Insert(reefast::ReeIdx),
    Place(reefast::ReeIdx, bool),
}
