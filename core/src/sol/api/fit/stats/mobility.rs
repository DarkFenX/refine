use crate::sol::{AttrVal, api::FitMut};

impl<'a> FitMut<'a> {
    pub fn get_agility_factor(&mut self) -> Option<AttrVal> {
        self.get_ship_mut().and_then(|mut v| v.get_agility_factor())
    }
    pub fn get_align_time(&mut self) -> Option<AttrVal> {
        self.get_ship_mut().and_then(|mut v| v.get_align_time())
    }
}
