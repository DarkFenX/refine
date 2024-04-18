use std::num::Wrapping;

use crate::{
    defs::SsFleetId,
    ss::fleet::SsFleet,
    util::{Error, ErrorKind, Result, StMap},
};

pub(in crate::ss) struct SsFleets {
    counter: Wrapping<SsFleetId>,
    data: StMap<SsFleetId, SsFleet>,
}
impl SsFleets {
    pub(in crate::ss) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: StMap::new(),
        }
    }
    pub(in crate::ss) fn add_fleet(&mut self) -> Result<SsFleetId> {
        let fleet_id = self.alloc_fleet_id()?;
        self.data.insert(fleet_id, SsFleet::new(fleet_id));
        Ok(fleet_id)
    }
    pub(in crate::ss) fn get_fleet(&self, fleet_id: &SsFleetId) -> Result<&SsFleet> {
        self.data
            .get(fleet_id)
            .ok_or_else(|| Error::new(ErrorKind::FleetNotFound(*fleet_id)))
    }
    pub(in crate::ss) fn get_fleet_mut(&mut self, fleet_id: &SsFleetId) -> Result<&mut SsFleet> {
        self.data
            .get_mut(fleet_id)
            .ok_or_else(|| Error::new(ErrorKind::FleetNotFound(*fleet_id)))
    }
    pub(in crate::ss) fn remove_fleet(&mut self, fleet_id: &SsFleetId) -> Result<()> {
        match self.data.remove(fleet_id) {
            Some(_) => Ok(()),
            None => Err(Error::new(ErrorKind::FleetNotFound(*fleet_id))),
        }
    }
    pub(in crate::ss) fn iter_fleet_ids(&self) -> impl ExactSizeIterator<Item = &SsFleetId> {
        self.data.keys()
    }
    pub(in crate::ss) fn iter_fleets(&self) -> impl ExactSizeIterator<Item = &SsFleet> {
        self.data.values()
    }
    fn alloc_fleet_id(&mut self) -> Result<SsFleetId> {
        let start = self.counter;
        while self.data.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                return Err(Error::new(ErrorKind::FleetIdAllocFailed));
            }
        }
        let fleet_id = self.counter.0;
        self.counter += 1;
        Ok(fleet_id)
    }
}
