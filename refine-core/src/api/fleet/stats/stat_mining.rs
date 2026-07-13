use crate::{
    api::FleetMut,
    svc::{
        cycle::CseqMap,
        vast::{StatMining, StatMiningItemKinds, StatTimeOptions},
    },
};

impl<'s> FleetMut<'s> {
    pub fn get_stat_mps(
        &mut self,
        item_kinds: StatMiningItemKinds,
        time_options: StatTimeOptions,
        mission_ore: bool,
    ) -> StatMining {
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        self.sol.svc.get_stat_fits_mps(
            &mut CseqMap::new(),
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
            mission_ore,
        )
    }
}
