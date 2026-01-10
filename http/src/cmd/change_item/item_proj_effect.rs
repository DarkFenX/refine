use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde(default)]
    type_id: Option<i32>,
    #[serde(default)]
    state: Option<bool>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    add_projs: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_proj_effect = core_sol.get_proj_effect_mut(item_id).map_err(|error| match error {
            rc::err::GetProjEffectError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetProjEffectError::ItemIsNotProjEffect(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_proj_effect.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_proj_effect.set_state(state);
        }
        for projectee_item_id in self.add_projs.iter() {
            core_proj_effect
                .add_proj(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                })?;
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_proj_effect
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        apply_effect_modes(&mut core_proj_effect, &self.effect_modes);
        Ok(core_proj_effect.into())
    }
}
