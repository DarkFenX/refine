use crate::{
    ac, ad,
    sol::{
        svc::vast::{ValModuleStateModuleInfo, Vast},
        uad::item::{Item, ModuleState},
    },
};

impl Vast {
    pub(in crate::sol::svc) fn item_state_activated_loaded(&mut self, item: &Item, a_state: &ad::AState) {
        match a_state {
            ad::AState::Offline => {
                if let Item::Rig(rig) = item {
                    if let Some(val) = rig.get_a_attrs().unwrap().get(&ac::attrs::UPGRADE_COST) {
                        let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                        fit_data.rigs_offline_calibration.insert(rig.get_item_id(), *val);
                    }
                }
            }
            ad::AState::Online => match item {
                Item::Fighter(fighter) => {
                    let extras = fighter.get_a_extras().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    if extras.is_light_fighter {
                        fit_data.light_fighters_online.insert(fighter.get_item_id());
                    }
                    if extras.is_heavy_fighter {
                        fit_data.heavy_fighters_online.insert(fighter.get_item_id());
                    }
                    if extras.is_support_fighter {
                        fit_data.support_fighters_online.insert(fighter.get_item_id());
                    }
                    if extras.is_standup_light_fighter {
                        fit_data.standup_light_fighters_online.insert(fighter.get_item_id());
                    }
                    if extras.is_standup_heavy_fighter {
                        fit_data.standup_heavy_fighters_online.insert(fighter.get_item_id());
                    }
                    if extras.is_standup_support_fighter {
                        fit_data.standup_support_fighters_online.insert(fighter.get_item_id());
                    }
                }
                Item::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    fit_data.mods_svcs_online.insert(module.get_item_id());
                    if let Some(a_item_grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(a_item_grp_id, module.get_item_id());
                        if module.get_a_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_ONLINE) {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(module.get_item_id(), a_item_grp_id);
                        }
                    }
                    if let Some(sec_class) = extras.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(module.get_item_id(), sec_class);
                    }
                    if let ad::AState::Offline = extras.max_state {
                        fit_data.mods_state.insert(
                            module.get_item_id(),
                            ValModuleStateModuleInfo {
                                state: ModuleState::Online,
                                max_state: ModuleState::Offline,
                            },
                        );
                    }
                }
                Item::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_id()).unwrap();
                    let extras = service.get_a_extras().unwrap();
                    fit_data.mods_svcs_online.insert(service.get_item_id());
                    if let Some(a_item_grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(a_item_grp_id, service.get_item_id());
                        if service
                            .get_a_attrs()
                            .unwrap()
                            .contains_key(&ac::attrs::MAX_GROUP_ONLINE)
                        {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(service.get_item_id(), a_item_grp_id);
                        }
                    }
                    if let Some(sec_class) = extras.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(service.get_item_id(), sec_class);
                    }
                }
                _ => (),
            },
            ad::AState::Active => match item {
                Item::Charge(charge) => {
                    let extras = charge.get_a_extras().unwrap();
                    if extras.sec_zone_limitable {
                        let fit_data = self.get_fit_data_mut(&charge.get_fit_id()).unwrap();
                        fit_data.sec_zone_active.insert(charge.get_item_id());
                    }
                }
                Item::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    if let Some(a_item_grp_id) = extras.val_active_group_id {
                        fit_data
                            .mods_max_group_active_all
                            .add_entry(a_item_grp_id, module.get_item_id());
                        if module.get_a_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_ACTIVE) {
                            fit_data
                                .mods_max_group_active_limited
                                .insert(module.get_item_id(), a_item_grp_id);
                        }
                    }
                    match extras.max_state {
                        ad::AState::Offline => {
                            fit_data.mods_state.get_mut(&module.get_item_id()).unwrap().state = ModuleState::Active;
                        }
                        ad::AState::Online => {
                            fit_data.mods_state.insert(
                                module.get_item_id(),
                                ValModuleStateModuleInfo {
                                    state: ModuleState::Active,
                                    max_state: ModuleState::Online,
                                },
                            );
                        }
                        _ => (),
                    }
                    if extras.sec_zone_limitable {
                        fit_data.sec_zone_active.insert(module.get_item_id());
                    }
                    fit_data.mods_active.insert(module.get_item_id());
                }
                _ => (),
            },
            ad::AState::Overload => {
                if let Item::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    match extras.max_state {
                        ad::AState::Offline | ad::AState::Online => {
                            fit_data.mods_state.get_mut(&module.get_item_id()).unwrap().state = ModuleState::Overload;
                        }
                        ad::AState::Active => {
                            fit_data.mods_state.insert(
                                module.get_item_id(),
                                ValModuleStateModuleInfo {
                                    state: ModuleState::Overload,
                                    max_state: ModuleState::Active,
                                },
                            );
                        }
                        _ => (),
                    }
                    if let Some(td_lvl) = extras.overload_td_lvl {
                        fit_data.overload_td_lvl.insert(module.get_item_id(), td_lvl);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn item_state_deactivated_loaded(&mut self, item: &Item, a_state: &ad::AState) {
        match a_state {
            ad::AState::Offline => {
                if let Item::Rig(rig) = item {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                    fit_data.rigs_offline_calibration.remove(&rig.get_item_id());
                }
            }
            ad::AState::Online => match item {
                Item::Fighter(fighter) => {
                    let extras = fighter.get_a_extras().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    if extras.is_light_fighter {
                        fit_data.light_fighters_online.remove(&fighter.get_item_id());
                    }
                    if extras.is_heavy_fighter {
                        fit_data.heavy_fighters_online.remove(&fighter.get_item_id());
                    }
                    if extras.is_support_fighter {
                        fit_data.support_fighters_online.remove(&fighter.get_item_id());
                    }
                    if extras.is_standup_light_fighter {
                        fit_data.standup_light_fighters_online.remove(&fighter.get_item_id());
                    }
                    if extras.is_standup_heavy_fighter {
                        fit_data.standup_heavy_fighters_online.remove(&fighter.get_item_id());
                    }
                    if extras.is_standup_support_fighter {
                        fit_data.standup_support_fighters_online.remove(&fighter.get_item_id());
                    }
                }
                Item::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    fit_data.mods_svcs_online.remove(&module.get_item_id());
                    if let Some(a_item_grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(&a_item_grp_id, &module.get_item_id());
                        fit_data
                            .mods_svcs_max_group_online_limited
                            .remove(&module.get_item_id());
                    }
                    if extras.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(&module.get_item_id());
                    }
                    if let ad::AState::Offline = extras.max_state {
                        fit_data.mods_state.remove(&module.get_item_id());
                    }
                }
                Item::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_id()).unwrap();
                    let extras = service.get_a_extras().unwrap();
                    fit_data.mods_svcs_online.remove(&service.get_item_id());
                    if let Some(a_item_grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(&a_item_grp_id, &service.get_item_id());
                        fit_data
                            .mods_svcs_max_group_online_limited
                            .remove(&service.get_item_id());
                    }
                    if extras.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(&service.get_item_id());
                    }
                }
                _ => (),
            },
            ad::AState::Active => match item {
                Item::Charge(charge) => {
                    let fit_data = self.get_fit_data_mut(&charge.get_fit_id()).unwrap();
                    fit_data.sec_zone_active.remove(&charge.get_item_id());
                }
                Item::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    if let Some(a_item_grp_id) = extras.val_active_group_id {
                        fit_data
                            .mods_max_group_active_all
                            .remove_entry(&a_item_grp_id, &module.get_item_id());
                        fit_data.mods_max_group_active_limited.remove(&module.get_item_id());
                    }
                    match extras.max_state {
                        ad::AState::Offline => {
                            fit_data.mods_state.get_mut(&module.get_item_id()).unwrap().state = ModuleState::Online;
                        }
                        ad::AState::Online => {
                            fit_data.mods_state.remove(&module.get_item_id());
                        }
                        _ => (),
                    }
                    if extras.sec_zone_limitable {
                        fit_data.sec_zone_active.remove(&module.get_item_id());
                    }
                    fit_data.mods_active.remove(&module.get_item_id());
                }
                _ => (),
            },
            ad::AState::Overload => {
                if let Item::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    match extras.max_state {
                        ad::AState::Offline | ad::AState::Online => {
                            fit_data.mods_state.get_mut(&module.get_item_id()).unwrap().state = ModuleState::Active;
                        }
                        ad::AState::Active => {
                            fit_data.mods_state.remove(&module.get_item_id());
                        }
                        _ => (),
                    }
                    if extras.overload_td_lvl.is_some() {
                        fit_data.overload_td_lvl.remove(&module.get_item_id());
                    }
                }
            }
            _ => (),
        }
    }
}
