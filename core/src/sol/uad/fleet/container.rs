use std::num::Wrapping;

use crate::{
    err::basic::FleetFoundError,
    sol::{FleetId, uad::fleet::Fleet},
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol) struct Fleets {
    counter: Wrapping<FleetId>,
    data: StMap<FleetId, Fleet>,
}
impl Fleets {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: StMap::new(),
        }
    }
    pub(in crate::sol) fn add_fleet(&mut self) -> FleetId {
        let fleet_id = self.alloc_fleet_id();
        self.data.insert(fleet_id, Fleet::new(fleet_id));
        fleet_id
    }
    pub(in crate::sol) fn get_fleet(&self, fleet_id: &FleetId) -> Result<&Fleet, FleetFoundError> {
        self.data.get(fleet_id).ok_or_else(|| FleetFoundError::new(*fleet_id))
    }
    pub(in crate::sol) fn get_fleet_mut(&mut self, fleet_id: &FleetId) -> Result<&mut Fleet, FleetFoundError> {
        self.data
            .get_mut(fleet_id)
            .ok_or_else(|| FleetFoundError::new(*fleet_id))
    }
    pub(in crate::sol) fn remove_fleet(&mut self, fleet_id: &FleetId) -> Result<(), FleetFoundError> {
        match self.data.remove(fleet_id) {
            Some(_) => Ok(()),
            None => Err(FleetFoundError::new(*fleet_id)),
        }
    }
    pub(in crate::sol) fn iter_fleets(&self) -> impl ExactSizeIterator<Item = &Fleet> {
        self.data.values()
    }
    fn alloc_fleet_id(&mut self) -> FleetId {
        let start = self.counter;
        while self.data.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                panic!("ran out of fleet ID space");
            }
        }
        let fleet_id = self.counter.0;
        self.counter += 1;
        fleet_id
    }
}
