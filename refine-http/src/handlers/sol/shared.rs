use serde::Deserialize;

use crate::err::ApiError;

#[derive(Deserialize)]
pub(crate) struct SolParams {
    #[serde(default)]
    sol: rs::SolInfoMode,
    #[serde(default)]
    fleet: rs::FleetInfoMode,
    #[serde(default)]
    fit: rs::FitInfoMode,
    #[serde(default)]
    item: rs::ItemInfoMode,
}
impl SolParams {
    pub(super) fn into_cmd(self) -> rs::SolInfoCmd {
        rs::SolInfoCmd::new()
            .with_sol(self.sol)
            .with_fleet_default(self.fleet)
            .with_fit_default(self.fit)
            .with_item_default(self.item)
    }
    pub(super) fn into_cmd_br(self) -> rs::SolInfoCmdBr {
        rs::SolInfoCmdBr::new()
            .with_sol(self.sol)
            .with_fleet_default(self.fleet)
            .with_fit_default(self.fit)
            .with_item_default(self.item)
    }
}

pub(super) fn parse_src_alias_from_body(src_alias: Option<String>) -> Result<Option<rs::src::SrcAlias>, ApiError> {
    match src_alias {
        Some(src_alias) => match rs::src::SrcAlias::try_pruned(&src_alias) {
            Ok(src_alias) => Ok(Some(src_alias)),
            Err(err) => Err(ApiError::BodySrcParse(src_alias, err)),
        },
        None => Ok(None),
    }
}
