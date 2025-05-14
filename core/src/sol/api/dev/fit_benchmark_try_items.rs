use std::hint::black_box;

use crate::sol::{ItemTypeId, api::FitMut, svc::vast::ValOptions};

impl<'a> FitMut<'a> {
    pub fn benchmark_try_items(&mut self, type_ids: &[ItemTypeId], val_options: &ValOptions, iterations: usize) {
        for _ in 0..iterations {
            black_box(self.sol.internal_try_fit_items(self.key, type_ids, val_options));
        }
    }
}
