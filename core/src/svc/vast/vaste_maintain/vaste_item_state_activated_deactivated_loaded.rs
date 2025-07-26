use crate::{
    ac, ad,
    misc::ModuleState,
    svc::vast::{ValModuleStateModuleInfo, Vast},
    ud::{UItem, UItemKey},
};

impl Vast {
    pub(in crate::svc) fn item_state_activated_loaded(
        &mut self,
        item_key: UItemKey,
        item: &UItem,
        a_state: &ad::AState,
    ) {
        match a_state {
            ad::AState::Offline => {
                if let UItem::Rig(rig) = item
                    && let Some(val) = rig.get_attrs().unwrap().get(&ac::attrs::UPGRADE_COST)
                {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_key());
                    fit_data.rigs_offline_calibration.insert(item_key, *val);
                }
            }
            ad::AState::Online => match item {
                UItem::Fighter(fighter) => {
                    let item_axt = fighter.get_axt().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_key());
                    if item_axt.is_light_fighter {
                        fit_data.light_fighters_online.insert(item_key);
                    }
                    if item_axt.is_heavy_fighter {
                        fit_data.heavy_fighters_online.insert(item_key);
                    }
                    if item_axt.is_support_fighter {
                        fit_data.support_fighters_online.insert(item_key);
                    }
                    if item_axt.is_st_light_fighter {
                        fit_data.st_light_fighters_online.insert(item_key);
                    }
                    if item_axt.is_st_heavy_fighter {
                        fit_data.st_heavy_fighters_online.insert(item_key);
                    }
                    if item_axt.is_st_support_fighter {
                        fit_data.st_support_fighters_online.insert(item_key);
                    }
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let item_axt = module.get_axt().unwrap();
                    fit_data.mods_svcs_online.insert(item_key);
                    if let Some(a_item_grp_id) = module.get_val_online_group_id() {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(a_item_grp_id, item_key);
                        if module.get_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_ONLINE) {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(item_key, a_item_grp_id);
                        }
                    }
                    if let Some(sec_class) = item_axt.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(item_key, sec_class);
                    }
                    if let ad::AState::Offline = module.get_max_state().unwrap() {
                        fit_data.mods_state.insert(
                            item_key,
                            ValModuleStateModuleInfo {
                                state: ModuleState::Online,
                                max_state: ModuleState::Offline,
                            },
                        );
                    }
                }
                UItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_key());
                    let item_axt = service.get_axt().unwrap();
                    fit_data.mods_svcs_online.insert(item_key);
                    if let Some(a_item_grp_id) = service.get_val_online_group_id() {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(a_item_grp_id, item_key);
                        if service.get_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_ONLINE) {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(item_key, a_item_grp_id);
                        }
                    }
                    if let Some(sec_class) = item_axt.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(item_key, sec_class);
                    }
                }
                _ => (),
            },
            ad::AState::Active => match item {
                UItem::Charge(charge) => {
                    let item_axt = charge.get_axt().unwrap();
                    if item_axt.sec_zone_limitable {
                        let fit_data = self.get_fit_data_mut(&charge.get_fit_key());
                        fit_data.sec_zone_active.insert(item_key);
                    }
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let item_axt = module.get_axt().unwrap();
                    if let Some(a_item_grp_id) = module.get_val_active_group_id() {
                        fit_data.mods_max_group_active_all.add_entry(a_item_grp_id, item_key);
                        if module.get_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_ACTIVE) {
                            fit_data.mods_max_group_active_limited.insert(item_key, a_item_grp_id);
                        }
                    }
                    match module.get_max_state().unwrap() {
                        ad::AState::Offline => {
                            fit_data.mods_state.get_mut(&item_key).unwrap().state = ModuleState::Active;
                        }
                        ad::AState::Online => {
                            fit_data.mods_state.insert(
                                item_key,
                                ValModuleStateModuleInfo {
                                    state: ModuleState::Active,
                                    max_state: ModuleState::Online,
                                },
                            );
                        }
                        _ => (),
                    }
                    if item_axt.sec_zone_limitable {
                        fit_data.sec_zone_active.insert(item_key);
                    }
                    fit_data.mods_active.insert(item_key);
                }
                _ => (),
            },
            ad::AState::Overload => {
                if let UItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let item_axt = module.get_axt().unwrap();
                    match module.get_max_state().unwrap() {
                        ad::AState::Offline | ad::AState::Online => {
                            fit_data.mods_state.get_mut(&item_key).unwrap().state = ModuleState::Overload;
                        }
                        ad::AState::Active => {
                            fit_data.mods_state.insert(
                                item_key,
                                ValModuleStateModuleInfo {
                                    state: ModuleState::Overload,
                                    max_state: ModuleState::Active,
                                },
                            );
                        }
                        _ => (),
                    }
                    if let Some(td_lvl) = item_axt.overload_td_lvl {
                        fit_data.overload_td_lvl.insert(item_key, td_lvl);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn item_state_deactivated_loaded(
        &mut self,
        item_key: &UItemKey,
        item: &UItem,
        a_state: &ad::AState,
    ) {
        match a_state {
            ad::AState::Offline => {
                if let UItem::Rig(rig) = item {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_key());
                    fit_data.rigs_offline_calibration.remove(item_key);
                }
            }
            ad::AState::Online => match item {
                UItem::Fighter(fighter) => {
                    let item_axt = fighter.get_axt().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_key());
                    if item_axt.is_light_fighter {
                        fit_data.light_fighters_online.remove(item_key);
                    }
                    if item_axt.is_heavy_fighter {
                        fit_data.heavy_fighters_online.remove(item_key);
                    }
                    if item_axt.is_support_fighter {
                        fit_data.support_fighters_online.remove(item_key);
                    }
                    if item_axt.is_st_light_fighter {
                        fit_data.st_light_fighters_online.remove(item_key);
                    }
                    if item_axt.is_st_heavy_fighter {
                        fit_data.st_heavy_fighters_online.remove(item_key);
                    }
                    if item_axt.is_st_support_fighter {
                        fit_data.st_support_fighters_online.remove(item_key);
                    }
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let item_axt = module.get_axt().unwrap();
                    fit_data.mods_svcs_online.remove(item_key);
                    if let Some(a_item_grp_id) = module.get_val_online_group_id() {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(&a_item_grp_id, item_key);
                        fit_data.mods_svcs_max_group_online_limited.remove(item_key);
                    }
                    if item_axt.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(item_key);
                    }
                    if let ad::AState::Offline = module.get_max_state().unwrap() {
                        fit_data.mods_state.remove(item_key);
                    }
                }
                UItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_key());
                    let item_axt = service.get_axt().unwrap();
                    fit_data.mods_svcs_online.remove(item_key);
                    if let Some(a_item_grp_id) = service.get_val_online_group_id() {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(&a_item_grp_id, item_key);
                        fit_data.mods_svcs_max_group_online_limited.remove(item_key);
                    }
                    if item_axt.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(item_key);
                    }
                }
                _ => (),
            },
            ad::AState::Active => match item {
                UItem::Charge(charge) => {
                    let fit_data = self.get_fit_data_mut(&charge.get_fit_key());
                    fit_data.sec_zone_active.remove(item_key);
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let item_axt = module.get_axt().unwrap();
                    if let Some(a_item_grp_id) = module.get_val_active_group_id() {
                        fit_data
                            .mods_max_group_active_all
                            .remove_entry(&a_item_grp_id, item_key);
                        fit_data.mods_max_group_active_limited.remove(item_key);
                    }
                    match module.get_max_state().unwrap() {
                        ad::AState::Offline => {
                            fit_data.mods_state.get_mut(item_key).unwrap().state = ModuleState::Online;
                        }
                        ad::AState::Online => {
                            fit_data.mods_state.remove(item_key);
                        }
                        _ => (),
                    }
                    if item_axt.sec_zone_limitable {
                        fit_data.sec_zone_active.remove(item_key);
                    }
                    fit_data.mods_active.remove(item_key);
                }
                _ => (),
            },
            ad::AState::Overload => {
                if let UItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let item_axt = module.get_axt().unwrap();
                    match module.get_max_state().unwrap() {
                        ad::AState::Offline | ad::AState::Online => {
                            fit_data.mods_state.get_mut(item_key).unwrap().state = ModuleState::Active;
                        }
                        ad::AState::Active => {
                            fit_data.mods_state.remove(item_key);
                        }
                        _ => (),
                    }
                    if item_axt.overload_td_lvl.is_some() {
                        fit_data.overload_td_lvl.remove(item_key);
                    }
                }
            }
            _ => (),
        }
    }
}
