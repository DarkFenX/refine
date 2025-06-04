use std::hint::black_box;

use crate::sol::{
    ItemTypeId,
    api::FitMut,
    svc::vast::{IntValOptions, ValOptions},
};

impl<'a> FitMut<'a> {
    pub fn benchmark_try_items(&mut self, type_ids: &[ItemTypeId], val_options: &ValOptions, iterations: usize) {
        let int_val_options = IntValOptions::from_pub_options(self.sol, val_options);
        for _ in 0..iterations {
            black_box(self.sol.internal_try_fit_items(self.key, type_ids, &int_val_options));
        }
    }
}
