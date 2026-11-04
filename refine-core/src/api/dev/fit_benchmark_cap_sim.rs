use itertools::Itertools;

use crate::{
    Fit, UnitInterval,
    stats::StatCapSimStagger,
    svc::{cycle::CseqMap, vast::StatCapSimStaggerInt},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn benchmark_cap_sim(&mut self, staggers: &[StatCapSimStagger], iterations: usize) {
        let ship_uid = self.core.u_data.fits.get(self.uid).ship.unwrap();
        let staggers = match staggers.is_empty() {
            true => vec![StatCapSimStaggerInt::from_pub(
                self.core,
                &StatCapSimStagger::new(false),
            )],
            false => staggers
                .iter()
                .map(|stagger| StatCapSimStaggerInt::from_pub(self.core, stagger))
                .collect_vec(),
        };
        let mut reuse_cseq_map = CseqMap::new();
        for _ in 0..iterations {
            for stagger in staggers.iter() {
                self.core
                    .svc
                    .get_stat_item_cap_sim(
                        &mut reuse_cseq_map,
                        &self.core.u_data,
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
