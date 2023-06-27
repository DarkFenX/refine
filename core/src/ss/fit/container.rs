use std::{collections::HashMap, num::Wrapping};

use crate::{
    defs::SsFitId,
    ss::fit::SsFit,
    util::{Error, ErrorKind, Result},
};

pub(in crate::ss) struct SsFits {
    counter: Wrapping<SsFitId>,
    data: HashMap<SsFitId, SsFit>,
}
impl SsFits {
    pub(in crate::ss) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: HashMap::new(),
        }
    }
    pub(in crate::ss) fn add_fit(&mut self) -> Result<SsFitId> {
        let fit_id = self.alloc_fit_id()?;
        self.data.insert(fit_id, SsFit::new(fit_id));
        Ok(fit_id)
    }
    pub(in crate::ss) fn get_fit(&self, fit_id: &SsFitId) -> Result<&SsFit> {
        self.data
            .get(fit_id)
            .ok_or_else(|| Error::new(ErrorKind::FitNotFound(*fit_id)))
    }
    pub(in crate::ss) fn get_fit_mut(&mut self, fit_id: &SsFitId) -> Result<&mut SsFit> {
        self.data
            .get_mut(fit_id)
            .ok_or_else(|| Error::new(ErrorKind::FitNotFound(*fit_id)))
    }
    pub(in crate::ss) fn remove_fit(&mut self, fit_id: &SsFitId) -> Result<()> {
        match self.data.remove(fit_id) {
            Some(_) => Ok(()),
            None => Err(Error::new(ErrorKind::FitNotFound(*fit_id))),
        }
    }
    pub(in crate::ss) fn get_fit_ids(&self) -> Vec<SsFitId> {
        self.data.keys().map(|v| *v).collect()
    }
    pub(in crate::ss) fn check_fit(&self, fit_id: &SsFitId) -> Result<()> {
        match self.data.contains_key(&fit_id) {
            true => Ok(()),
            false => Err(Error::new(ErrorKind::FitNotFound(*fit_id))),
        }
    }
    fn alloc_fit_id(&mut self) -> Result<SsFitId> {
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
