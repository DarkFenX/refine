use crate::{
    err::basic::ProjFoundError,
    misc::ProjRange,
    sol::{SolarSystem, api::RangedProjMut},
    ud::{UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_projection_range(
        &mut self,
        projector_key: UItemKey,
        projectee_key: UItemKey,
        range: ProjRange,
    ) -> Result<(), ProjFoundError> {
        let projector_u_item = self.u_data.items.get(projector_key);
        match projector_u_item {
            UItem::Drone(_) => self.internal_set_drone_proj_range(projector_key, projectee_key, range),
            UItem::Fighter(_) => self.internal_set_fighter_proj_range(projector_key, projectee_key, range),
            UItem::Module(_) => self.internal_set_module_proj_range(projector_key, projectee_key, range),
            _ => unreachable!("un-range-projectable item kind is used in projection"),
        }
    }
}

impl<'a> RangedProjMut<'a> {
    pub fn set_range(&mut self, range: ProjRange) {
        self.sol
            .internal_set_projection_range(self.projector_key, self.projectee_key, range)
            .unwrap()
    }
}
