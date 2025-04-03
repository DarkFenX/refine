use std::num::Wrapping;

use crate::{
    err::basic::FitFoundError,
    sol::{FitId, uad::fit::Fit},
    util::HMap,
};

#[derive(Clone)]
pub(in crate::sol) struct Fits {
    counter: Wrapping<FitId>,
    data: HMap<FitId, Fit>,
}
impl Fits {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: HMap::new(),
        }
    }
    pub(in crate::sol) fn add_fit(&mut self) -> FitId {
        let fit_id = self.alloc_fit_id();
        self.data.insert(fit_id, Fit::new(fit_id));
        fit_id
    }
    pub(in crate::sol) fn get_fit(&self, fit_id: &FitId) -> Result<&Fit, FitFoundError> {
        self.data.get(fit_id).ok_or(FitFoundError { fit_id: *fit_id })
    }
    pub(in crate::sol) fn get_fit_mut(&mut self, fit_id: &FitId) -> Result<&mut Fit, FitFoundError> {
        self.data.get_mut(fit_id).ok_or(FitFoundError { fit_id: *fit_id })
    }
    pub(in crate::sol) fn remove_fit(&mut self, fit_id: &FitId) -> Result<Fit, FitFoundError> {
        match self.data.remove(fit_id) {
            Some(fit) => Ok(fit),
            None => Err(FitFoundError { fit_id: *fit_id }),
        }
    }
    pub(in crate::sol) fn iter_fit_ids(&self) -> impl ExactSizeIterator<Item = &FitId> {
        self.data.keys()
    }
    pub(in crate::sol) fn iter_fits(&self) -> impl ExactSizeIterator<Item = &Fit> {
        self.data.values()
    }
    pub(in crate::sol) fn iter_fits_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Fit> {
        self.data.values_mut()
    }
    pub(in crate::sol) fn contains_err(&self, fit_id: &FitId) -> Result<(), FitFoundError> {
        match self.data.contains_key(fit_id) {
            true => Ok(()),
            false => Err(FitFoundError { fit_id: *fit_id }),
        }
    }
    pub(in crate::sol) fn len(&self) -> usize {
        self.data.len()
    }
    fn alloc_fit_id(&mut self) -> FitId {
        let start = self.counter;
        while self.data.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                panic!("ran out of fit ID space");
            }
        }
        let fit_id = self.counter.0;
        self.counter += 1;
        fit_id
    }
}
