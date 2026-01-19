use serde::Deserialize;

use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeRigCmd {
    #[serde(default)]
    type_id: Option<i32>,
    #[serde(default)]
    state: Option<bool>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeRigCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_rig = core_sol.get_rig_mut(item_id).map_err(|error| match error {
            rc::err::GetRigError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetRigError::ItemIsNotRig(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_rig.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_rig.set_state(state);
        }
        apply_effect_modes(&mut core_rig, &self.effect_modes);
        Ok(HItemIdsResp::from_core_rig(core_rig))
    }
}
