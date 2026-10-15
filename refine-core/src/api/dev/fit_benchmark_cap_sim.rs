use itertools::Itertools;

use crate::{
    api::FitMut,
    num::UnitInterval,
    svc::{
        cycle::CseqMap,
        vast::{StatCapSimStagger, StatCapSimStaggerInt},
    },
};

impl<'s> FitMut<'s> {
    pub fn benchmark_cap_sim(&mut self, staggers: &[StatCapSimStagger], iterations: usize) {
        let ship_uid = self.sol.u_data.fits.get(self.uid).ship.unwrap();
        let staggers = match staggers.is_empty() {
            true => vec![StatCapSimStaggerInt::from_pub(self.sol, &StatCapSimStagger::new(false))],
            false => staggers
                .iter()
                .map(|stagger| StatCapSimStaggerInt::from_pub(self.sol, stagger))
                .collect_vec(),
        };
        let mut reuse_cseq_map = CseqMap::new();
        for _ in 0..iterations {
            for stagger in staggers.iter() {
                self.sol
                    .svc
                    .get_stat_item_cap_sim(
                        &mut reuse_cseq_map,
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
