use std::sync::Arc;

use parking_lot::{Mutex, MutexGuard};

pub(in crate::dev) struct SolCloner<'a> {
    inner: Arc<Mutex<SolClonerInner<'a>>>,
}
impl<'a> SolCloner<'a> {
    pub(in crate::dev) fn new(original: &'a rc::SolOwned) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SolClonerInner::new(original))),
        }
    }
    pub(in crate::dev) fn lock(&'a self) -> MutexGuard<'a, SolClonerInner<'a>> {
        self.inner.lock()
    }
}

pub(in crate::dev) struct SolClonerInner<'a> {
    original: &'a rc::SolOwned,
    allocated: Vec<Box<rc::SolOwned>>,
}
impl<'a> SolClonerInner<'a> {
    fn new(original: &'a rc::SolOwned) -> Self {
        Self {
            original,
            allocated: Vec::with_capacity(rayon::current_num_threads()),
        }
    }
    pub(in crate::dev) fn take(&mut self) -> Box<rc::SolOwned> {
        match self.allocated.pop() {
            Some(sol) => sol,
            None => Box::new(self.original.clone()),
        }
    }
    pub(in crate::dev) fn put(&mut self, sol: Box<rc::SolOwned>) {
        self.allocated.push(sol);
    }
}
