use std::hint::black_box;

use itertools::Itertools;

use crate::{
    api::{FitMut, ItemTypeId},
    svc::vast::{ValOptions, ValOptionsInt},
    ud::UEffectUpdates,
};

impl<'a> FitMut<'a> {
    pub fn benchmark_try_items(&mut self, type_ids: &[ItemTypeId], val_options: &ValOptions, iterations: usize) {
        let type_aids = type_ids.into_iter().map(|v| v.into_aid()).collect_vec();
        let int_val_options = ValOptionsInt::from_pub(self.sol, val_options);
        let mut reuse_eupdates = UEffectUpdates::new();
        for _ in 0..iterations {
            black_box(
                self.sol
                    .internal_try_fit_items(self.uid, &type_aids, &int_val_options, &mut reuse_eupdates),
            );
        }
    }
}
