use crate::{
    defs::{EEffectId, SolItemId},
    err::basic::{ItemFoundError, ItemKindMatchError, SideEffectError},
    sol::{view::SolView, SolEffectMode, SolarSystem},
};

impl SolarSystem {
    pub fn set_booster_side_effect_state(
        &mut self,
        item_id: &SolItemId,
        effect_id: &EEffectId,
        state: bool,
    ) -> Result<(), SetBoosterSideEffectStateError> {
        let booster = self.items.get_item_mut(item_id)?.get_booster_mut()?;
        let a_item = booster.get_a_item().map_err(|_| SideEffectError::new(*effect_id))?;
        if !a_item.effect_datas.contains_key(effect_id) {
            return Err(SideEffectError::new(*effect_id).into());
        }
        let effect = match self.src.get_a_effect(effect_id) {
            Some(effect) => effect,
            None => return Err(SideEffectError::new(*effect_id).into()),
        };
        if effect.chance_attr_id.is_none() {
            return Err(SideEffectError::new(*effect_id).into());
        }
        let effect_state = match state {
            true => SolEffectMode::StateCompliance,
            false => SolEffectMode::FullCompliance,
        };
        booster.get_effect_modes_mut().set(*effect_id, effect_state);
        let item = self.items.get_item(item_id).unwrap();
        self.svcs.process_effects(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            item,
            item.get_state(),
        );
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetBoosterSideEffectStateError {
    ItemNotFound(ItemFoundError),
    ItemIsNotBooster(ItemKindMatchError),
    NotSideEffect(SideEffectError),
}
impl From<ItemFoundError> for SetBoosterSideEffectStateError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for SetBoosterSideEffectStateError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotBooster(error)
    }
}
impl std::error::Error for SetBoosterSideEffectStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotBooster(e) => Some(e),
            Self::NotSideEffect(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetBoosterSideEffectStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotBooster(e) => e.fmt(f),
            Self::NotSideEffect(e) => e.fmt(f),
        }
    }
}
impl From<SideEffectError> for SetBoosterSideEffectStateError {
    fn from(error: SideEffectError) -> Self {
        Self::NotSideEffect(error)
    }
}
