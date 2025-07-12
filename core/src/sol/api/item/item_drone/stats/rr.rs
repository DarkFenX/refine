use crate::{def::AttrVal, misc::Spool, sol::api::DroneMut};

impl<'a> DroneMut<'a> {
    pub fn get_stat_rr_shield(&mut self, spool: Option<Spool>, ignore_state: bool) -> Option<AttrVal> {
        self.sol
            .svc
            .get_stat_item_orr_shield(&self.sol.uad, &self.sol.reffs, self.key, spool, ignore_state)
    }
}
