use itertools::Itertools;

use crate::{Fit, UnitInterval, stats::StatCapSimStagger, svc::vast::StatCapSimStaggerInt};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn benchmark_cap_sim(&mut self, staggers: &[StatCapSimStagger], iterations: usize) {
        let ship_uid = self.sol.u_data.fits.get(self.uid).ship.unwrap();
        let staggers = match staggers.is_empty() {
            true => vec![StatCapSimStaggerInt::from_pub(self.sol, &StatCapSimStagger::new(false))],
            false => staggers
                .iter()
                .map(|stagger| StatCapSimStaggerInt::from_pub(self.sol, stagger))
                .collect_vec(),
        };
        for _ in 0..iterations {
            // Cache is cleared per iteration, since that's how stats requests are normally used
            self.sol.reuse.cseq_cache.clear();
            for stagger in staggers.iter() {
                self.sol
                    .svc
                    .get_stat_item_cap_sim(
                        &mut self.sol.reuse.cseq_cache,
                        &self.sol.u_data,
                        ship_uid,
                        UnitInterval::ONE,
                        None,
                        stagger,
                        None,
                    )
                    .unwrap();
            }
        }
    }
}
