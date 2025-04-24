use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeCharacterCmd {
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
        if let Some(state) = self.state {
            core_character.set_state(state);
        }
        apply_effect_modes(&mut core_character, &self.effect_modes);
        Ok(core_character.into())
    }
}
