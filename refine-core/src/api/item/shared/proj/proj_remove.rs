use crate::{
    SolarSystem,
    err::basic::ProjFoundError,
    ud::{UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_projection(
        &mut self,
        projector_uid: UItemId,
        projectee_uid: UItemId,
    ) -> Result<(), ProjFoundError> {
        let projector_u_item = self.u_data.items.get(projector_uid);
        match projector_u_item {
            UItem::Drone(..) => self.internal_remove_drone_proj(projector_uid, projectee_uid),
            UItem::Fighter(..) => self.internal_remove_fighter_proj(projector_uid, projectee_uid),
            UItem::Module(..) => self.internal_remove_module_proj(projector_uid, projectee_uid),
            // Still need to handle projected effect, even if projected effect is not using ranged
            // projections - this method is used not just by ranged projection removal
            UItem::ProjEffect(..) => self.internal_remove_proj_effect_proj(projector_uid, projectee_uid),
            _ => unreachable!("unprojectable item kind is used in projection"),
        }
    }
}
