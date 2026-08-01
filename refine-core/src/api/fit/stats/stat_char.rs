use crate::{
    api::{FitMut, ItemMutCommon, StatFitCharacterError},
    num::PValue,
};

impl<'s> FitMut<'s> {
    pub fn get_stat_drone_control_range(&mut self) -> Result<PValue, StatFitCharacterError<!>> {
        Ok(self.get_character_for_stats()?.get_stat_drone_control_range()?)
    }
}
