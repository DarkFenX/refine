use crate::{
    api::RangedProjMut,
    err::basic::ProjFoundError,
    sol::SolarSystem,
    ud::{UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_projection(
        &mut self,
        projector_key: UItemId,
        projectee_key: UItemId,
    ) -> Result<(), ProjFoundError> {
        let projector_u_item = self.u_data.items.get(projector_key);
        match projector_u_item {
            UItem::Drone(_) => self.internal_remove_drone_proj(projector_key, projectee_key),
            UItem::Fighter(_) => self.internal_remove_fighter_proj(projector_key, projectee_key),
            UItem::Module(_) => self.internal_remove_module_proj(projector_key, projectee_key),
            // Still need to handle projected effect, even if projected effect is not using ranged
            // projections - this method is used not just by ranged projection removal
            UItem::ProjEffect(_) => self.internal_remove_proj_effect_proj(projector_key, projectee_key),
            _ => unreachable!("unprojectable item kind is used in projection"),
        }
    }
}

impl<'a> RangedProjMut<'a> {
    pub fn remove(self) {
        self.sol
            .internal_remove_projection(self.projector_key, self.projectee_key)
            .unwrap()
    }
}
