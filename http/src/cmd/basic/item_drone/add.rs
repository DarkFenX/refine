use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::{
        HCmdResps, HItemIdsResp,
        shared::{HEffectModeMap, HFitIdBackref, HItemIdBackref, HMutationOnAdd, get_primary_fit},
    },
    shared::{HCoordinates, HMinionState, HMovement, HNpcProp},
    util::HExecError,
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HDroneAddCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HDroneAddCmdICtxBIds,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HDroneAddCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    ictx_cmd: HDroneAddCmdICtxRIds,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HDroneAddCmdICtxBIds {
    #[serde(flatten)]
    shared: HDroneAddCmdShared,
    #[serde(default)]
    proj_item_ids: Vec<HItemIdBackref>,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HDroneAddCmdICtxRIds {
    #[serde(flatten)]
    shared: HDroneAddCmdShared,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    proj_item_ids: Vec<rc::ItemId>,
}
#[derive(Deserialize)]
struct HDroneAddCmdShared {
    type_id: i32,
    state: HMinionState,
    mutation: Option<HMutationOnAdd>,
    npc_prop: Option<HNpcProp>,
    coordinates: Option<HCoordinates>,
    movement: Option<HMovement>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HDroneAddCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HDroneAddCmdFCtxRIds, HExecError> {
        Ok(HDroneAddCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl HDroneAddCmdICtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HDroneAddCmdICtxRIds, HExecError> {
        Ok(HDroneAddCmdICtxRIds {
            shared: self.shared,
            proj_item_ids: resps.render_item_ids(self.proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HDroneAddCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HDroneAddCmdICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_drone = core_fit.add_drone(
            rc::ItemTypeId::from_i32(self.shared.type_id),
            self.shared.state.into_core(),
            self.shared.coordinates.map(|v| v.into_core()),
            self.shared.movement.map(|v| v.into_core()),
        );
        if let Some(h_mutation) = self.shared.mutation.as_ref() {
            match h_mutation {
                HMutationOnAdd::Short(mutator_id) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(*mutator_id);
                    core_drone.mutate(core_mutator_id).unwrap();
                }
                HMutationOnAdd::Full(h_full_mutation) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(h_full_mutation.mutator_id);
                    let core_mutation = core_drone.mutate(core_mutator_id).unwrap();
                    h_full_mutation.apply_attrs_on_add(core_mutation);
                }
            }
        }
        if let Some(h_npc_prop) = self.shared.npc_prop {
            core_drone.set_npc_prop(Some(h_npc_prop.into_core()))
        }
        for projectee_item_id in self.proj_item_ids.iter() {
            core_drone.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        if let Some(h_effect_modes) = self.shared.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_drone);
        }
        Ok(HItemIdsResp::from_core_drone(core_drone))
    }
}
