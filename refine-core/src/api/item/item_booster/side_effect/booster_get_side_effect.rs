use super::shared::get_se_chance_attr_aid_by_effect_rid;
use crate::{Booster, EffectId, ItemId, SideEffect};

impl<'s> Booster<'s> {
    pub fn get_side_effect(&mut self, effect_id: &EffectId) -> Result<SideEffect<'_>, SideEffectGetError> {
        let effect_aid = effect_id.into_aid();
        let r_data = &self.sol.u_data.r_data;
        let chance_attr_aid = r_data
            .get_effect_rid_by_aid(&effect_aid)
            .and_then(|effect_rid| get_se_chance_attr_aid_by_effect_rid(r_data, effect_rid));
        match chance_attr_aid {
            Some(chance_attr_aid) => Ok(SideEffect::new(self.sol, self.uid, effect_aid, chance_attr_aid)),
            None => Err(SideEffectGetError::SideEffectNotFound(
                self.sol.u_data.items.ext_id_by_int_id(self.uid),
                *effect_id,
            )),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SideEffectGetError {
    #[error("effect {1} is not a side effect on item {0}")]
    SideEffectNotFound(ItemId, EffectId),
}
