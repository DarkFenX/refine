use crate::api::{Autocharge, EntityIter, Fighter};

impl<'s> Fighter<'s> {
    pub fn iter_autocharges(&mut self) -> EntityIter<'_, Autocharge<'_>> {
        let u_fighter = self.sol.u_data.items.get(self.uid).dc_fighter().unwrap();
        let autocharge_uids = u_fighter.get_autocharges().values().collect();
        EntityIter::new(self.sol, autocharge_uids)
    }
}
