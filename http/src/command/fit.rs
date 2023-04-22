#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub(crate) enum FitCommand {
    SetShip(SetShipCmd),
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SetShipCmd {
    pub(crate) ship_type_id: reefast::ReeInt,
}
