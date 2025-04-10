use std::hint::black_box;

use crate::sol::{FitId, ItemTypeId, SolarSystem, svc::vast::ValOptions};

impl SolarSystem {
    pub fn benchmark_try_fit_items(
        &mut self,
        fit_id: &FitId,
        type_ids: &[ItemTypeId],
        val_options: &ValOptions,
        iterations: usize,
    ) {
        let fit_key = self.uad.fits.key_by_id(fit_id).unwrap();
        for _ in 0..iterations {
            black_box(self.try_fit_items_internal(fit_key, type_ids, &val_options));
        }
    }
}
