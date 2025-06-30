use crate::{
    def::ItemKey,
    err::basic::ProjFoundError,
    sol::{SolarSystem, api::RangedProjMut},
    uad::UadItem,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_projection(
        &mut self,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), ProjFoundError> {
        let projector_uad_item = self.uad.items.get(projector_item_key);
        match projector_uad_item {
            UadItem::Drone(_) => self.internal_remove_drone_proj(projector_item_key, projectee_item_key),
            UadItem::Fighter(_) => self.internal_remove_fighter_proj(projector_item_key, projectee_item_key),
            UadItem::Module(_) => self.internal_remove_module_proj(projector_item_key, projectee_item_key),
            // Still need to handle projected effect, even if projected effect is not using ranged
            // projections - this method is used not just by ranged projection removal
            UadItem::ProjEffect(_) => self.internal_remove_proj_effect_proj(projector_item_key, projectee_item_key),
            _ => unreachable!("unprojectable item kind is used in projection"),
        }
    }
}

impl<'a> RangedProjMut<'a> {
    pub fn remove(self) {
        self.sol
            .internal_remove_projection(self.projector_item_key, self.projectee_item_key)
            .unwrap()
    }
}
