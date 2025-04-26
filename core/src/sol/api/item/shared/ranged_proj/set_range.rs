use crate::{
    err::basic::ProjFoundError,
    sol::{AttrVal, ItemKey, SolarSystem, api::RangedProjMut, uad::item::UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_projection_range(
        &mut self,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), ProjFoundError> {
        let projector_uad_item = self.uad.items.get(projector_item_key);
        match projector_uad_item {
            UadItem::Drone(_) => self.internal_change_drone_proj(projector_item_key, projectee_item_key, range),
            UadItem::Fighter(_) => self.internal_change_fighter_proj(projector_item_key, projectee_item_key, range),
            UadItem::Module(_) => self.internal_change_module_proj(projector_item_key, projectee_item_key, range),
            _ => panic!(),
        }
    }
}

impl<'a> RangedProjMut<'a> {
    pub fn set_range(&mut self, range: Option<AttrVal>) {
        self.sol
            .internal_set_projection_range(self.projector_item_key, self.projectee_item_key, range)
            .unwrap()
    }
}
