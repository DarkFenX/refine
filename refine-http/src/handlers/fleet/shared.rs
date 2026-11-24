use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct FleetParams {
    #[serde(default)]
    fleet: rs::info::FleetInfoMode,
    #[serde(default)]
    fit: rs::info::FitInfoMode,
    #[serde(default)]
    item: rs::info::ItemInfoMode,
}
impl FleetParams {
    pub(super) fn into_cmd(self) -> rs::info::FleetInfoCmd {
        rs::info::FleetInfoCmd::new()
            .with_fleet(self.fleet)
            .with_fit_default(self.fit)
            .with_item_default(self.item)
    }
}
