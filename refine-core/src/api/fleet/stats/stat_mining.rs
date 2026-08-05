use crate::{
    api::FleetMut,
    svc::{
        cycle::CseqMap,
        vast::{StatMining, StatMiningItemKinds, StatMiningResourceKind, StatTimeOptions},
    },
};

impl<'s> FleetMut<'s> {
    pub fn get_stat_mps(
        &mut self,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
        resource_kind: StatMiningResourceKind,
    ) -> StatMining {
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        self.sol.svc.get_stat_fits_mps(
            &mut CseqMap::new(),
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
            resource_kind,
        )
    }
}
