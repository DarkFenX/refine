use itertools::Itertools;

use crate::{
    defs::SolItemId,
    sol::{
        item::{SolItem, SolItemState},
        SolView, SolarSystem,
    },
};

impl SolarSystem {
    pub(in crate::sol::sole_item) fn add_item_id_to_svcs(&mut self, item_id: &SolItemId) {
        let item = self.items.get_item(&item_id).unwrap();
        self.svcs
            .add_item(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item);
    }
    pub(in crate::sol::sole_item) fn change_item_id_state_in_svcs(
        &mut self,
        item_id: &SolItemId,
        old_state: SolItemState,
        new_state: SolItemState,
    ) {
        if new_state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.switch_item_state(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                item,
                old_state,
                new_state,
            );
        }
    }
    pub(in crate::sol::sole_item) fn remove_incoming_projections(&mut self, item_id: &SolItemId) {
        let proj_incoming = self.proj_tracker.iter_projectors(item_id).map(|v| *v).collect_vec();
        for proj_item_id in proj_incoming.iter() {
            let proj_item = self.items.get_item(proj_item_id).unwrap();
            match proj_item {
                // TODO: add drone proj
                SolItem::Module(_) => self.remove_module_proj(proj_item_id, item_id).unwrap(),
                SolItem::Fighter(_) => self.remove_fighter_proj(proj_item_id, item_id).unwrap(),
                SolItem::ProjEffect(_) => self.remove_proj_effect_proj(proj_item_id, item_id).unwrap(),
                _ => panic!(),
            }
        }
    }
}
