use crate::{
    def::{AttrVal, ItemId, ItemKey},
    sol::SolarSystem,
};

/// Projection which allows to set range.
pub struct RangedProj<'a> {
    pub(in crate::sol::api) sol: &'a SolarSystem,
    pub(in crate::sol::api) projector_item_key: ItemKey,
    pub(in crate::sol::api) projectee_item_key: ItemKey,
}
impl<'a> RangedProj<'a> {
    pub(in crate::sol::api) fn new(
        sol: &'a SolarSystem,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Self {
        Self {
            sol,
            projector_item_key,
            projectee_item_key,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.uad.items.id_by_key(self.projectee_item_key)
    }
    pub fn get_range(&self) -> Option<AttrVal> {
        get_range(self.sol, self.projector_item_key, &self.projectee_item_key)
    }
}

/// Projection which allows to set range.
pub struct RangedProjMut<'a> {
    pub(in crate::sol::api) sol: &'a mut SolarSystem,
    pub(in crate::sol::api) projector_item_key: ItemKey,
    pub(in crate::sol::api) projectee_item_key: ItemKey,
}
impl<'a> RangedProjMut<'a> {
    pub(in crate::sol::api) fn new(
        sol: &'a mut SolarSystem,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Self {
        Self {
            sol,
            projector_item_key,
            projectee_item_key,
        }
    }
    pub fn get_projectee_item_id(&self) -> ItemId {
        self.sol.uad.items.id_by_key(self.projectee_item_key)
    }
    pub fn get_range(&self) -> Option<AttrVal> {
        get_range(self.sol, self.projector_item_key, &self.projectee_item_key)
    }
}

fn get_range(sol: &SolarSystem, projector_item_key: ItemKey, projectee_item_key: &ItemKey) -> Option<AttrVal> {
    sol.uad
        .items
        .get(projector_item_key)
        .get_projs()
        .unwrap()
        .get(projectee_item_key)
        .unwrap()
        .map(|v| v.c2c)
}
