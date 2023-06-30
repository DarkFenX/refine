use crate::{
    cmd::{item, HCmdResp},
    shared::HState,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFighterCmd {
    type_id: rc::EItemId,
    state: HState,
}
impl HAddFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem, fit_id: &rc::SsFitId) -> rc::Result<HCmdResp> {
        Ok(core_ss.add_fighter(*fit_id, self.type_id, (&self.state).into())?.into())
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde(with = "crate::util::serde_string")]
    item_id: rc::SsItemId,
    #[serde(flatten)]
    item_cmd: item::HChangeFighterCmd,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        self.item_cmd.execute(core_ss, &self.item_id)
    }
}
