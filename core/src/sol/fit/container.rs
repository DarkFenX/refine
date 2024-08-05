use std::num::Wrapping;

use crate::{
    defs::SolFitId,
    sol::fit::SolFit,
    util::{Error, ErrorKind, Result, StMap},
};

#[derive(Clone)]
pub(in crate::sol) struct SolFits {
    counter: Wrapping<SolFitId>,
    data: StMap<SolFitId, SolFit>,
}
impl SolFits {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: StMap::new(),
        }
    }
    pub(in crate::sol) fn add_fit(&mut self) -> Result<SolFitId> {
        let fit_id = self.alloc_fit_id()?;
        self.data.insert(fit_id, SolFit::new(fit_id));
        Ok(fit_id)
    }
    pub(in crate::sol) fn get_fit(&self, fit_id: &SolFitId) -> Result<&SolFit> {
        self.data
            .get(fit_id)
            .ok_or_else(|| Error::new(ErrorKind::FitNotFound(*fit_id)))
    }
    pub(in crate::sol) fn get_fit_mut(&mut self, fit_id: &SolFitId) -> Result<&mut SolFit> {
        self.data
            .get_mut(fit_id)
            .ok_or_else(|| Error::new(ErrorKind::FitNotFound(*fit_id)))
    }
    pub(in crate::sol) fn remove_fit(&mut self, fit_id: &SolFitId) -> Result<()> {
        match self.data.remove(fit_id) {
            Some(_) => Ok(()),
            None => Err(Error::new(ErrorKind::FitNotFound(*fit_id))),
        }
    }
    pub(in crate::sol) fn iter_fit_ids(&self) -> impl ExactSizeIterator<Item = &SolFitId> {
        self.data.keys()
    }
    pub(in crate::sol) fn iter_fits(&self) -> impl ExactSizeIterator<Item = &SolFit> {
        self.data.values()
    }
    pub(in crate::sol) fn iter_fits_mut(&mut self) -> impl ExactSizeIterator<Item = &mut SolFit> {
        self.data.values_mut()
    }
    pub(in crate::sol) fn len(&self) -> usize {
        self.data.len()
    }
    fn alloc_fit_id(&mut self) -> Result<SolFitId> {
        let start = self.counter;
        while self.data.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                return Err(Error::new(ErrorKind::FitIdAllocFailed));
            }
        }
        let fit_id = self.counter.0;
        self.counter += 1;
        Ok(fit_id)
    }
}
