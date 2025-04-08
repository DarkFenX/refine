use std::num::Wrapping;

use slab::Slab;

use crate::{
    err::basic::FleetFoundError,
    sol::{FleetId, FleetKey, uad::fleet::Fleet},
    util::RMap,
};

#[derive(Clone)]
pub(in crate::sol) struct Fleets {
    counter: Wrapping<FleetId>,
    pub(super) data: Slab<Fleet>,
    pub(super) id_to_key: RMap<FleetId, FleetKey>,
}
impl Fleets {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: Slab::with_capacity(2),
            id_to_key: RMap::with_capacity(2),
        }
    }
    pub(in crate::sol) fn alloc_fleet_id(&mut self) -> FleetId {
        let start = self.counter;
        while self.id_to_key.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                panic!("ran out of fleet ID space");
            }
        }
        let fleet_id = self.counter.0;
        self.counter += 1;
        fleet_id
    }
    pub(in crate::sol) fn add(&mut self, fleet: Fleet) -> FleetKey {
        let fleet_id = fleet.id;
        let fleet_key = self.data.insert(fleet);
        self.id_to_key.insert(fleet_id, fleet_key);
        fleet_key
    }
    pub(in crate::sol) fn key_by_id(&self, fleet_id: &FleetId) -> Option<FleetKey> {
        self.id_to_key.get(fleet_id).copied()
    }
    pub(in crate::sol) fn key_by_id_err(&self, fleet_id: &FleetId) -> Result<FleetKey, FleetFoundError> {
        match self.id_to_key.get(fleet_id) {
            Some(fleet_key) => Ok(*fleet_key),
            None => Err(FleetFoundError { fleet_id: *fleet_id }),
        }
    }
    pub(in crate::sol) fn id_by_key(&self, fleet_key: FleetKey) -> FleetId {
        self.get(fleet_key).id
    }
    pub(in crate::sol) fn try_get(&self, fleet_key: FleetKey) -> Option<&Fleet> {
        self.data.get(fleet_key)
    }
    pub(in crate::sol) fn get(&self, fleet_key: FleetKey) -> &Fleet {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get(fleet_key).unwrap()
    }
    pub(in crate::sol) fn get_mut(&mut self, fleet_key: FleetKey) -> &mut Fleet {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_mut(fleet_key).unwrap()
    }
    pub(in crate::sol) fn remove(&mut self, fleet_key: FleetKey) -> Fleet {
        // Keys are supposed to be valid throughout whole lib, so use non-try removal
        let fleet = self.data.remove(fleet_key);
        self.id_to_key.remove(&fleet.id);
        fleet
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = (FleetKey, &Fleet)> {
        self.data.iter()
    }
    pub(in crate::sol) fn keys(&self) -> impl ExactSizeIterator<Item = FleetKey> {
        self.data.iter().map(|(key, _)| key)
    }
    pub(in crate::sol) fn values(&self) -> impl ExactSizeIterator<Item = &Fleet> {
        self.data.iter().map(|(_, fleet)| fleet)
    }
    pub(in crate::sol) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Fleet> {
        self.data.iter_mut().map(|(_, fleet)| fleet)
    }
}
