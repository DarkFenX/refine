use std::sync::Arc;

use parking_lot::{Mutex, MutexGuard};

pub(in crate::cmd) struct HSolCloner<'a> {
    inner: Arc<Mutex<HSolClonerInner<'a>>>,
}
impl<'a> HSolCloner<'a> {
    pub(in crate::cmd) fn new(original: &'a rc::SolarSystem) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HSolClonerInner::new(original))),
        }
    }
    pub(in crate::cmd) fn lock(&'a self) -> MutexGuard<'a, HSolClonerInner<'a>> {
        self.inner.lock()
    }
}

pub(in crate::cmd) struct HSolClonerInner<'a> {
    original: &'a rc::SolarSystem,
    allocated: Vec<rc::SolarSystem>,
}
impl<'a> HSolClonerInner<'a> {
    fn new(original: &'a rc::SolarSystem) -> Self {
        Self {
            original,
            allocated: Vec::new(),
        }
    }
    pub(in crate::cmd) fn get(&mut self) -> rc::SolarSystem {
        match self.allocated.pop() {
            Some(sol) => sol,
            None => self.original.clone(),
        }
    }
    pub(in crate::cmd) fn put(&mut self, sol: rc::SolarSystem) {
        self.allocated.push(sol);
    }
}
