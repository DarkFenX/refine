use crate::{
    ad, ec,
    sol::{
        svc::vast::{SolModuleStateValFail, SolVast},
        uad::item::{SolItem, SolItemState, SolModuleState},
    },
};

impl SolVast {
    pub(in crate::sol::svc) fn item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        match state {
            SolItemState::Online => match item {
                SolItem::Module(module) => {
                    let extras = module.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_online_group_id {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data.mods_max_group_online_all.add_entry(grp_id, module.get_id());
                        if module.get_attrs().unwrap().contains_key(&ec::attrs::MAX_GROUP_ONLINE) {
                            fit_data.mods_max_group_online_limited.insert(module.get_id(), grp_id);
                        }
                    }
                    if let ad::AState::Offline = extras.max_state {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data.mods_state.insert(
                            module.get_id(),
                            SolModuleStateValFail::new(
                                module.get_id(),
                                SolModuleState::Online,
                                SolModuleState::Offline,
                            ),
                        );
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
                    let extras = module.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_active_group_id {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data.mods_max_group_active_all.add_entry(grp_id, module.get_id());
                        if module.get_attrs().unwrap().contains_key(&ec::attrs::MAX_GROUP_ACTIVE) {
                            fit_data.mods_max_group_active_limited.insert(module.get_id(), grp_id);
                        }
                    }
                    match extras.max_state {
                        ad::AState::Offline => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_state.get_mut(&module.get_id()).unwrap().state = SolModuleState::Active;
                        }
                        ad::AState::Online => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_state.insert(
                                module.get_id(),
                                SolModuleStateValFail::new(
                                    module.get_id(),
                                    SolModuleState::Active,
                                    SolModuleState::Online,
                                ),
                            );
                        }
                        _ => (),
                    }
                }
            }
            SolItemState::Overload => {
                if let SolItem::Module(module) = item {
                    match module.get_a_extras().unwrap().max_state {
                        ad::AState::Offline | ad::AState::Online => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_state.get_mut(&module.get_id()).unwrap().state = SolModuleState::Overload;
                        }
                        ad::AState::Active => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_state.insert(
                                module.get_id(),
                                SolModuleStateValFail::new(
                                    module.get_id(),
                                    SolModuleState::Overload,
                                    SolModuleState::Active,
                                ),
                            );
                        }
                        _ => (),
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
                    let extras = module.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_online_group_id {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data
                            .mods_max_group_online_all
                            .remove_entry(&grp_id, &module.get_id());
                        fit_data.mods_max_group_online_limited.remove(&module.get_id());
                    }
                    if let ad::AState::Offline = extras.max_state {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data.mods_state.remove(&module.get_id());
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
                    let extras = module.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_active_group_id {
                        let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                        fit_data
                            .mods_max_group_active_all
                            .remove_entry(&grp_id, &module.get_id());
                        fit_data.mods_max_group_active_limited.remove(&module.get_id());
                    }
                    match extras.max_state {
                        ad::AState::Offline => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_state.get_mut(&module.get_id()).unwrap().state = SolModuleState::Online;
                        }
                        ad::AState::Online => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_state.remove(&module.get_id());
                        }
                        _ => (),
                    }
                }
            }
            SolItemState::Overload => {
                if let SolItem::Module(module) = item {
                    match module.get_a_extras().unwrap().max_state {
                        ad::AState::Offline | ad::AState::Online => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_state.get_mut(&module.get_id()).unwrap().state = SolModuleState::Active;
                        }
                        ad::AState::Active => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_state.remove(&module.get_id());
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}
