use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct FleetParams {
    #[serde(default)]
    fleet: rs::FleetInfoMode,
}
impl FleetParams {
    pub(super) fn into_cmd(self) -> rs::FleetInfoCmd {
        rs::FleetInfoCmd::new().with_fleet(self.fleet)
    }
}
