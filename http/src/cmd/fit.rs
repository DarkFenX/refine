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

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum State {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl Into<reefast::State> for State {
    fn into(self) -> reefast::State {
        match self {
            Self::Offline => reefast::State::Offline,
            Self::Online => reefast::State::Online,
            Self::Active => reefast::State::Active,
            Self::Ghost => reefast::State::Ghost,
            Self::Overload => reefast::State::Overload,
        }
    }
}

#[derive(Copy, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum AddMode {
    Append,
    Equip,
    Insert(reefast::ReeIdx),
    Place(reefast::ReeIdx, bool),
}
impl Into<reefast::OrdAddMode> for AddMode {
    fn into(self) -> reefast::OrdAddMode {
        match self {
            Self::Append => reefast::OrdAddMode::Append,
            Self::Equip => reefast::OrdAddMode::Equip,
            Self::Insert(i) => reefast::OrdAddMode::Insert(i),
            Self::Place(i, r) => reefast::OrdAddMode::Place(i, r),
        }
    }
}
