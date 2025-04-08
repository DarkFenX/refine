use std::num::Wrapping;

use slab::Slab;

use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, uad::fit::Fit},
    util::RMap,
};

#[derive(Clone)]
pub(in crate::sol) struct Fits {
    counter: Wrapping<FitId>,
    pub(super) data: Slab<Fit>,
    pub(super) id_to_key: RMap<FitId, FitKey>,
}
impl Fits {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: Slab::with_capacity(10),
            id_to_key: RMap::with_capacity(10),
        }
    }
    pub(in crate::sol) fn alloc_fit_id(&mut self) -> FitId {
        let start = self.counter;
        while self.id_to_key.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                panic!("ran out of fit ID space");
            }
        }
        let fit_id = self.counter.0;
        self.counter += 1;
        fit_id
    }
    pub(in crate::sol) fn add(&mut self, fit: Fit) -> FitKey {
        let fit_id = fit.id;
        let fit_key = self.data.insert(fit);
        self.id_to_key.insert(fit_id, fit_key);
        fit_key
    }
    pub(in crate::sol) fn key_by_id(&self, fit_id: &FitId) -> Option<FitKey> {
        self.id_to_key.get(fit_id).copied()
    }
    pub(in crate::sol) fn key_by_id_err(&self, fit_id: &FitId) -> Result<FitKey, FitFoundError> {
        match self.id_to_key.get(fit_id) {
            Some(fit_key) => Ok(*fit_key),
            None => Err(FitFoundError { fit_id: *fit_id }),
        }
    }
    pub(in crate::sol) fn id_by_key(&self, fit_key: FitKey) -> FitId {
        self.get(fit_key).id
    }
    pub(in crate::sol) fn try_get(&self, fit_key: FitKey) -> Option<&Fit> {
        self.data.get(fit_key)
    }
    pub(in crate::sol) fn get(&self, fit_key: FitKey) -> &Fit {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get(fit_key).unwrap()
    }
    pub(in crate::sol) fn get_mut(&mut self, fit_key: FitKey) -> &mut Fit {
        // Keys are supposed to be valid throughout whole lib, so just unwrap
        self.data.get_mut(fit_key).unwrap()
    }
    pub(in crate::sol) fn remove(&mut self, fit_key: FitKey) -> Fit {
        // Keys are supposed to be valid throughout whole lib, so use non-try removal
        let fit = self.data.remove(fit_key);
        self.id_to_key.remove(&fit.id);
        fit
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = (FitKey, &Fit)> {
        self.data.iter()
    }
    pub(in crate::sol) fn keys(&self) -> impl ExactSizeIterator<Item = FitKey> {
        self.data.iter().map(|(key, _)| key)
    }
    pub(in crate::sol) fn values(&self) -> impl ExactSizeIterator<Item = &Fit> {
        self.data.iter().map(|(_, fit)| fit)
    }
    pub(in crate::sol) fn values_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Fit> {
        self.data.iter_mut().map(|(_, fit)| fit)
    }
    pub(in crate::sol) fn len(&self) -> usize {
        self.data.len()
    }
}
