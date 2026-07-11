use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    cmd::shared::{
        HAddMode, HCmdResps, HCreatedItemIdsResp, HEffectModeMap, HFitIdBackref, HItemIdBackref, HMutationOnAdd,
        get_primary_fit,
    },
    err::HExecError,
    shared::{HModRack, HModuleState, HOptionalReload, HSpool},
};

// Commands with full context
#[derive(Deserialize)]
pub(crate) struct HModuleAddCmdFCtxBIds {
    fit_id: HFitIdBackref,
    #[serde(flatten)]
    ictx_cmd: HModuleAddCmdICtxBIds,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HModuleAddCmdFCtxRIds {
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde(flatten)]
    ictx_cmd: HModuleAddCmdICtxRIds,
}

// Commands with incomplete context
#[derive(Deserialize)]
pub(crate) struct HModuleAddCmdICtxBIds {
    #[serde(flatten)]
    shared: HModuleAddCmdShared,
    #[serde(default)]
    proj_item_ids: Vec<HItemIdBackref>,
}
#[serde_as]
#[derive(Deserialize)]
pub(crate) struct HModuleAddCmdICtxRIds {
    #[serde(flatten)]
    shared: HModuleAddCmdShared,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    #[serde(default)]
    proj_item_ids: Vec<rc::ItemId>,
}
#[derive(Deserialize)]
struct HModuleAddCmdShared {
    rack: HModRack,
    add_mode: HAddMode,
    type_id: i32,
    state: HModuleState,
    mutation: Option<HMutationOnAdd>,
    charge_type_id: Option<i32>,
    spool: Option<HSpool>,
    optional_reload: Option<HOptionalReload>,
    effect_modes: Option<HEffectModeMap>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HModuleAddCmdFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HModuleAddCmdFCtxRIds, HExecError> {
        Ok(HModuleAddCmdFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl HModuleAddCmdICtxBIds {
    pub(in crate::cmd) fn render(self, resps: &HCmdResps) -> Result<HModuleAddCmdICtxRIds, HExecError> {
        Ok(HModuleAddCmdICtxRIds {
            shared: self.shared,
            proj_item_ids: resps.render_item_ids(self.proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HModuleAddCmdFCtxRIds {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCreatedItemIdsResp, HExecError> {
        self.ictx_cmd.execute(core_sol, &self.fit_id)
    }
}

impl HModuleAddCmdICtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fit_id: &rc::FitId,
    ) -> Result<HCreatedItemIdsResp, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut core_module = core_fit.create_module(
            self.shared.rack.into_core(),
            self.shared.add_mode.into_core(),
            rc::ItemTypeId::from_i32(self.shared.type_id),
            self.shared.state.into_core(),
        );
        if let Some(h_mutation) = self.shared.mutation.as_ref() {
            match h_mutation {
                HMutationOnAdd::Short(mutator_id) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(*mutator_id);
                    core_module.mutate(core_mutator_id).unwrap();
                }
                HMutationOnAdd::Full(h_full_mutation) => {
                    let core_mutator_id = rc::ItemTypeId::from_i32(h_full_mutation.mutator_id);
                    let core_mutation = core_module.mutate(core_mutator_id).unwrap();
                    h_full_mutation.apply_attrs_on_add(core_mutation);
                }
            }
        }
        if let Some(charge_type_id) = self.shared.charge_type_id {
            let core_charge_type_id = rc::ItemTypeId::from_i32(charge_type_id);
            core_module.set_charge_type_id(core_charge_type_id);
        }
        if let Some(h_spool) = self.shared.spool {
            core_module.set_spool(Some(h_spool.into_core()));
        }
        if let Some(h_optional_reload) = self.shared.optional_reload {
            core_module.set_optional_reload(Some(h_optional_reload.into_core()));
        }
        for projectee_item_id in self.proj_item_ids.iter() {
            core_module.add_proj(projectee_item_id).map_err(|error| match error {
                rc::err::AddProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                rc::err::AddProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                rc::err::AddProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
            })?;
        }
        if let Some(h_effect_modes) = self.shared.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_module);
        }
        Ok(HCreatedItemIdsResp::from_core_module(core_module))
    }
}
