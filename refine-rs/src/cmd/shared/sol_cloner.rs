use std::sync::Arc;

use parking_lot::{Mutex, MutexGuard};

pub(in crate::cmd) struct SolCloner<'a> {
    inner: Arc<Mutex<SolClonerInner<'a>>>,
}
impl<'a> SolCloner<'a> {
    pub(in crate::cmd) fn new(original: &'a rc::SolarSystem) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SolClonerInner::new(original))),
        }
    }
    pub(in crate::cmd) fn lock(&'a self) -> MutexGuard<'a, SolClonerInner<'a>> {
        self.inner.lock()
    }
}

pub(in crate::cmd) struct SolClonerInner<'a> {
    original: &'a rc::SolarSystem,
    #[allow(clippy::vec_box)]
    allocated: Vec<Box<rc::SolarSystem>>,
}
impl<'a> SolClonerInner<'a> {
    fn new(original: &'a rc::SolarSystem) -> Self {
        Self {
            original,
            allocated: Vec::with_capacity(tokio_rayon::rayon::current_num_threads()),
        }
    }
    pub(in crate::cmd) fn get(&mut self) -> Box<rc::SolarSystem> {
        match self.allocated.pop() {
            Some(sol) => sol,
            None => Box::new(self.original.clone()),
        }
    }
    pub(in crate::cmd) fn put(&mut self, sol: Box<rc::SolarSystem>) {
        self.allocated.push(sol);
    }
}
