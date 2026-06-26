use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        old_change_item::{
            HChangeAutochargeCmd, HChangeCharacterCmd, HChangeChargeCmd, HChangeFighterCmd, HChangeModuleCmd,
            HChangeProjEffectCmd, HChangeShipCmd, HChangeStanceCmd, HChangeSwEffectCmd,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeItemCommand {
    Autocharge(HChangeAutochargeCmd),
    Character(HChangeCharacterCmd),
    Charge(HChangeChargeCmd),
    Fighter(HChangeFighterCmd),
    Module(HChangeModuleCmd),
    ProjEffect(HChangeProjEffectCmd),
    Ship(HChangeShipCmd),
    Stance(HChangeStanceCmd),
    SwEffect(HChangeSwEffectCmd),
}
impl HChangeItemCommand {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        match self {
            Self::Autocharge(cmd) => cmd.execute(core_sol, item_id),
            Self::Character(cmd) => cmd.execute(core_sol, item_id),
            Self::Charge(cmd) => cmd.execute(core_sol, item_id),
            Self::Fighter(cmd) => cmd.execute(core_sol, item_id),
            Self::Module(cmd) => cmd.execute(core_sol, item_id),
            Self::ProjEffect(cmd) => cmd.execute(core_sol, item_id),
            Self::Ship(cmd) => cmd.execute(core_sol, item_id),
            Self::Stance(cmd) => cmd.execute(core_sol, item_id),
            Self::SwEffect(cmd) => cmd.execute(core_sol, item_id),
        }
    }
}
