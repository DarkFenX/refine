use crate::{
    Coordinates, CountNz, FighterCountInfo, FitId, ItemCommon, MinionState, Movement, RearmMinion,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UFighter, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Fighter<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Fighter<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_fighter(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> MinionState {
        get_u_fighter(self.core, self.uid).get_fighter_state()
    }
    pub fn get_count(&self) -> Option<FighterCountInfo> {
        get_u_fighter(self.core, self.uid).get_count_info()
    }
    pub fn get_count_override(&self) -> Option<CountNz> {
        get_u_fighter(self.core, self.uid).get_count_override()
    }
    pub fn get_coordinates(&self) -> Coordinates {
        Coordinates::from_xyz(get_u_fighter(self.core, self.uid).get_physics().coordinates)
    }
    pub fn get_movement(&self) -> Movement {
        Movement::from_u_physics(get_u_fighter(self.core, self.uid).get_physics())
    }
    pub fn get_rearm_minion(&self) -> RearmMinion {
        self.core.u_data.get_item_rearm_minion(self.uid, None)
    }
    pub fn get_rearm_minion_override(&self) -> Option<RearmMinion> {
        get_u_fighter(self.core, self.uid).get_rearm_minion_override()
    }
}
impl<'s> ItemCommon for Fighter<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fighter<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Fighter<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_core(&self) -> &CoreInner {
        self.core
    }
    fn get_core_mut(&mut self) -> &mut CoreInner {
        self.core
    }
}

fn get_u_fighter(core: &CoreInner, fighter_uid: UItemId) -> &UFighter {
    core.u_data.items.get(fighter_uid).dc_fighter().unwrap()
}
