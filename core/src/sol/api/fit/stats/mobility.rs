use crate::sol::{AttrVal, api::FitMut};

impl<'a> FitMut<'a> {
    pub fn get_stat_speed(&mut self) -> Option<AttrVal> {
        self.get_ship_mut().and_then(|mut v| v.get_speed())
    }
    pub fn get_stat_agility(&mut self) -> Option<AttrVal> {
        self.get_ship_mut().and_then(|mut v| v.get_agility())
    }
    pub fn get_stat_align_time(&mut self) -> Option<AttrVal> {
        self.get_ship_mut().and_then(|mut v| v.get_align_time())
    }
}
