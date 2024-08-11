use std::num::Wrapping;

use crate::{
    defs::SolFleetId,
    err::basic::{FleetAllocError, FleetFoundError},
    sol::fleet::SolFleet,
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol) struct SolFleets {
    counter: Wrapping<SolFleetId>,
    data: StMap<SolFleetId, SolFleet>,
}
impl SolFleets {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: StMap::new(),
        }
    }
    pub(in crate::sol) fn add_fleet(&mut self) -> Result<SolFleetId, FleetAllocError> {
        let fleet_id = self.alloc_fleet_id()?;
        self.data.insert(fleet_id, SolFleet::new(fleet_id));
        Ok(fleet_id)
    }
    pub(in crate::sol) fn get_fleet(&self, fleet_id: &SolFleetId) -> Result<&SolFleet, FleetFoundError> {
        self.data.get(fleet_id).ok_or_else(|| FleetFoundError::new(*fleet_id))
    }
    pub(in crate::sol) fn get_fleet_mut(&mut self, fleet_id: &SolFleetId) -> Result<&mut SolFleet, FleetFoundError> {
        self.data
            .get_mut(fleet_id)
            .ok_or_else(|| FleetFoundError::new(*fleet_id))
    }
    pub(in crate::sol) fn remove_fleet(&mut self, fleet_id: &SolFleetId) -> Result<(), FleetFoundError> {
        match self.data.remove(fleet_id) {
            Some(_) => Ok(()),
            None => Err(FleetFoundError::new(*fleet_id)),
        }
    }
    pub(in crate::sol) fn iter_fleet_ids(&self) -> impl ExactSizeIterator<Item = &SolFleetId> {
        self.data.keys()
    }
    pub(in crate::sol) fn iter_fleets(&self) -> impl ExactSizeIterator<Item = &SolFleet> {
        self.data.values()
    }
    fn alloc_fleet_id(&mut self) -> Result<SolFleetId, FleetAllocError> {
        let start = self.counter;
        while self.data.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                return Err(FleetAllocError::new());
            }
        }
        let fleet_id = self.counter.0;
        self.counter += 1;
        Ok(fleet_id)
    }
}
