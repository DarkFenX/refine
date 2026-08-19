use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct FleetParams {
    #[serde(default)]
    fleet: rs::FleetInfoMode,
    #[serde(default)]
    fit: rs::FitInfoMode,
    #[serde(default)]
    item: rs::ItemInfoMode,
}
impl FleetParams {
    pub(super) fn into_cmd(self) -> rs::FleetInfoCmd {
        rs::FleetInfoCmd::new()
            .with_fleet(self.fleet)
            .with_fit_default(self.fit)
            .with_item_default(self.item)
    }
}
