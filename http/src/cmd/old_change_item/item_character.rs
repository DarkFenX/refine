use serde::Deserialize;

use crate::{
    cmd::{HItemIdsResp, shared::HEffectModeMap},
    util::HExecError,
};

#[derive(Deserialize)]
pub(crate) struct HChangeCharacterCmd {
    type_id: Option<i32>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeCharacterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_character = core_sol.get_character_mut(item_id).map_err(|error| match error {
            rc::err::GetCharacterError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetCharacterError::ItemIsNotCharacter(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(type_id) = self.type_id {
            let core_type_id = rc::ItemTypeId::from_i32(type_id);
            core_character.set_type_id(core_type_id);
        }
        if let Some(state) = self.state {
            core_character.set_state(state);
        }
        if let Some(h_effect_modes) = self.effect_modes.as_ref() {
            h_effect_modes.apply(&mut core_character);
        }
        Ok(HItemIdsResp::from_core_character(core_character))
    }
}
