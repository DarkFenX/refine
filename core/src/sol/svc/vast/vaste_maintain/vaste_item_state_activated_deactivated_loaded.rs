use crate::{
    ec,
    sol::{
        svc::vast::SolVast,
        uad::item::{SolItem, SolItemState},
    },
};

impl SolVast {
    pub(in crate::sol::svc) fn item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        match state {
            SolItemState::Online => match item {
                SolItem::Module(module) => {
                    if let Some(grp_id) = module.get_a_extras().unwrap().val_online_group_id {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data.mods_max_group_online_all.add_entry(grp_id, module.get_id());
                        if module.get_attrs().unwrap().contains_key(&ec::attrs::MAX_GROUP_ONLINE) {
                            fit_data.mods_max_group_online_limited.insert(module.get_id(), grp_id);
                        }
                    }
                }
                SolItem::Fighter(fighter) => {
                    let extras = fighter.get_a_extras().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    if extras.is_light_fighter {
                        fit_data.light_fighters_online.insert(fighter.get_id());
                    }
                    if extras.is_heavy_fighter {
                        fit_data.heavy_fighters_online.insert(fighter.get_id());
                    }
                    if extras.is_support_fighter {
                        fit_data.support_fighters_online.insert(fighter.get_id());
                    }
                    if extras.is_standup_light_fighter {
                        fit_data.standup_light_fighters_online.insert(fighter.get_id());
                    }
                    if extras.is_standup_heavy_fighter {
                        fit_data.standup_heavy_fighters_online.insert(fighter.get_id());
                    }
                    if extras.is_standup_support_fighter {
                        fit_data.standup_support_fighters_online.insert(fighter.get_id());
                    }
                }
                _ => (),
            },
            SolItemState::Active => {
                if let SolItem::Module(module) = item {
                    if let Some(grp_id) = module.get_a_extras().unwrap().val_active_group_id {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data.mods_max_group_active_all.add_entry(grp_id, module.get_id());
                        if module.get_attrs().unwrap().contains_key(&ec::attrs::MAX_GROUP_ACTIVE) {
                            fit_data.mods_max_group_active_limited.insert(module.get_id(), grp_id);
                        }
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        match state {
            SolItemState::Online => match item {
                SolItem::Module(module) => {
                    if let Some(grp_id) = module.get_a_extras().unwrap().val_online_group_id {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data
                            .mods_max_group_online_all
                            .remove_entry(&grp_id, &module.get_id());
                        fit_data.mods_max_group_online_limited.remove(&module.get_id());
                    }
                }
                SolItem::Fighter(fighter) => {
                    let extras = fighter.get_a_extras().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    if extras.is_light_fighter {
                        fit_data.light_fighters_online.remove(&fighter.get_id());
                    }
                    if extras.is_heavy_fighter {
                        fit_data.heavy_fighters_online.remove(&fighter.get_id());
                    }
                    if extras.is_support_fighter {
                        fit_data.support_fighters_online.remove(&fighter.get_id());
                    }
                    if extras.is_standup_light_fighter {
                        fit_data.standup_light_fighters_online.remove(&fighter.get_id());
                    }
                    if extras.is_standup_heavy_fighter {
                        fit_data.standup_heavy_fighters_online.remove(&fighter.get_id());
                    }
                    if extras.is_standup_support_fighter {
                        fit_data.standup_support_fighters_online.remove(&fighter.get_id());
                    }
                }
                _ => (),
            },
            SolItemState::Active => {
                if let SolItem::Module(module) = item {
                    if let Some(grp_id) = module.get_a_extras().unwrap().val_active_group_id {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data
                            .mods_max_group_active_all
                            .remove_entry(&grp_id, &module.get_id());
                        fit_data.mods_max_group_active_limited.remove(&module.get_id());
                    }
                }
            }
            _ => (),
        }
    }
}
