use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, HProjDef, HProjDefFull, apply_effect_modes},
    },
    shared::HMinionState,
    util::{HExecError, TriStateField},
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde(default)]
    type_id: Option<rc::ItemTypeId>,
    #[serde(default)]
    state: Option<HMinionState>,
    #[serde(default)]
    count: TriStateField<rc::Count>,
    #[serde(default)]
    add_projs: Vec<HProjDef>,
    #[serde(default)]
    change_projs: Vec<HProjDefFull>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    #[serde(default)]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fighter = core_sol.get_fighter_mut(item_id).map_err(|error| match error {
            rc::err::GetFighterError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetFighterError::ItemIsNotFighter(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            core_fighter.set_type_id(type_id);
        }
        if let Some(state) = &self.state {
            core_fighter.set_state(state.into());
        }
        match self.count {
            TriStateField::Value(count) => {
                let fighter_count_override = rc::FighterCountOverride::new_checked(count)?;
                core_fighter.set_count_override(Some(fighter_count_override));
            }
            TriStateField::None => {
                core_fighter.set_count_override(None);
            }
            TriStateField::Absent => (),
        }
        for proj_def in self.add_projs.iter() {
            core_fighter
                .add_proj(
                    &proj_def.get_item_id(),
                    match proj_def.get_range() {
                        Some(range) => rc::ProjRange::S2S(range),
                        None => rc::ProjRange::None,
                    },
                )
                .map_err(|error| match error {
                    rc::err::AddRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddRangedProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddRangedProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                })?;
        }
        for proj_def in self.change_projs.iter() {
            core_fighter
                .get_proj_mut(&proj_def.get_item_id())
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .set_range(proj_def.get_range());
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_fighter
                .get_proj_mut(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::GetRangedProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::GetRangedProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?
                .remove();
        }
        apply_effect_modes(&mut core_fighter, &self.effect_modes);
        Ok(core_fighter.into())
    }
}
