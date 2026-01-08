use crate::{
    api::ModuleState,
    rd::RState,
    svc::vast::{ValModuleStateModuleInfo, Vast},
    ud::{UItem, UItemId},
};

impl Vast {
    pub(in crate::svc) fn item_state_activated_loaded(&mut self, item_uid: UItemId, item: &UItem, state: RState) {
        match state {
            RState::Offline => {
                if let UItem::Rig(rig) = item
                    && let Some(val) = rig.get_axt().unwrap().calibration_use
                {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_uid());
                    fit_data.rigs_offline_calibration.insert(item_uid, val);
                }
            }
            RState::Online => match item {
                UItem::Fighter(fighter) => {
                    let item_axt = fighter.get_axt().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_uid());
                    if item_axt.is_light_fighter {
                        fit_data.light_fighters_online.insert(item_uid);
                    }
                    if item_axt.is_heavy_fighter {
                        fit_data.heavy_fighters_online.insert(item_uid);
                    }
                    if item_axt.is_support_fighter {
                        fit_data.support_fighters_online.insert(item_uid);
                    }
                    if item_axt.is_st_light_fighter {
                        fit_data.st_light_fighters_online.insert(item_uid);
                    }
                    if item_axt.is_st_heavy_fighter {
                        fit_data.st_heavy_fighters_online.insert(item_uid);
                    }
                    if item_axt.is_st_support_fighter {
                        fit_data.st_support_fighters_online.insert(item_uid);
                    }
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                    let item_axt = module.get_axt().unwrap();
                    fit_data.mods_svcs_online.insert(item_uid);
                    if let Some(item_grp_aid) = module.get_val_online_group_id() {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(item_grp_aid, item_uid);
                        if item_axt.max_group_online_limited {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(item_uid, item_grp_aid);
                        }
                    }
                    if let Some(sec_class) = item_axt.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(item_uid, sec_class);
                    }
                    if let RState::Offline = module.get_max_state().unwrap() {
                        fit_data.mods_state.insert(
                            item_uid,
                            ValModuleStateModuleInfo {
                                state: ModuleState::Online,
                                max_state: ModuleState::Offline,
                            },
                        );
                    }
                }
                UItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_uid());
                    let item_axt = service.get_axt().unwrap();
                    fit_data.mods_svcs_online.insert(item_uid);
                    if let Some(item_grp_aid) = service.get_val_online_group_id() {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(item_grp_aid, item_uid);
                        if item_axt.max_group_online_limited {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(item_uid, item_grp_aid);
                        }
                    }
                    if let Some(sec_class) = item_axt.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(item_uid, sec_class);
                    }
                }
                _ => (),
            },
            RState::Active => match item {
                UItem::Charge(charge) => {
                    let item_axt = charge.get_axt().unwrap();
                    if item_axt.sec_zone_limitable {
                        let fit_data = self.get_fit_data_mut(&charge.get_fit_uid());
                        fit_data.sec_zone_active.insert(item_uid);
                    }
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                    let item_axt = module.get_axt().unwrap();
                    if let Some(item_grp_aid) = module.get_val_active_group_id() {
                        fit_data.mods_max_group_active_all.add_entry(item_grp_aid, item_uid);
                        if item_axt.max_group_active_limited {
                            fit_data.mods_max_group_active_limited.insert(item_uid, item_grp_aid);
                        }
                    }
                    match module.get_max_state().unwrap() {
                        RState::Offline => {
                            fit_data.mods_state.get_mut(&item_uid).unwrap().state = ModuleState::Active;
                        }
                        RState::Online => {
                            fit_data.mods_state.insert(
                                item_uid,
                                ValModuleStateModuleInfo {
                                    state: ModuleState::Active,
                                    max_state: ModuleState::Online,
                                },
                            );
                        }
                        _ => (),
                    }
                    if item_axt.sec_zone_limitable {
                        fit_data.sec_zone_active.insert(item_uid);
                    }
                    fit_data.mods_active.insert(item_uid);
                }
                _ => (),
            },
            RState::Overload => {
                if let UItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                    let item_axt = module.get_axt().unwrap();
                    match module.get_max_state().unwrap() {
                        RState::Offline | RState::Online => {
                            fit_data.mods_state.get_mut(&item_uid).unwrap().state = ModuleState::Overload;
                        }
                        RState::Active => {
                            fit_data.mods_state.insert(
                                item_uid,
                                ValModuleStateModuleInfo {
                                    state: ModuleState::Overload,
                                    max_state: ModuleState::Active,
                                },
                            );
                        }
                        _ => (),
                    }
                    if let Some(td_lvl) = item_axt.overload_td_lvl {
                        fit_data.overload_td_lvl.insert(item_uid, td_lvl);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc) fn item_state_deactivated_loaded(&mut self, item_uid: &UItemId, item: &UItem, a_state: RState) {
        match a_state {
            RState::Offline => {
                if let UItem::Rig(rig) = item {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_uid());
                    fit_data.rigs_offline_calibration.remove(item_uid);
                }
            }
            RState::Online => match item {
                UItem::Fighter(fighter) => {
                    let item_axt = fighter.get_axt().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_uid());
                    if item_axt.is_light_fighter {
                        fit_data.light_fighters_online.remove(item_uid);
                    }
                    if item_axt.is_heavy_fighter {
                        fit_data.heavy_fighters_online.remove(item_uid);
                    }
                    if item_axt.is_support_fighter {
                        fit_data.support_fighters_online.remove(item_uid);
                    }
                    if item_axt.is_st_light_fighter {
                        fit_data.st_light_fighters_online.remove(item_uid);
                    }
                    if item_axt.is_st_heavy_fighter {
                        fit_data.st_heavy_fighters_online.remove(item_uid);
                    }
                    if item_axt.is_st_support_fighter {
                        fit_data.st_support_fighters_online.remove(item_uid);
                    }
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                    let item_axt = module.get_axt().unwrap();
                    fit_data.mods_svcs_online.remove(item_uid);
                    if let Some(item_grp_aid) = module.get_val_online_group_id() {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(item_grp_aid, item_uid);
                        fit_data.mods_svcs_max_group_online_limited.remove(item_uid);
                    }
                    if item_axt.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(item_uid);
                    }
                    if let RState::Offline = module.get_max_state().unwrap() {
                        fit_data.mods_state.remove(item_uid);
                    }
                }
                UItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_uid());
                    let item_axt = service.get_axt().unwrap();
                    fit_data.mods_svcs_online.remove(item_uid);
                    if let Some(item_grp_aid) = service.get_val_online_group_id() {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(item_grp_aid, item_uid);
                        fit_data.mods_svcs_max_group_online_limited.remove(item_uid);
                    }
                    if item_axt.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(item_uid);
                    }
                }
                _ => (),
            },
            RState::Active => match item {
                UItem::Charge(charge) => {
                    let fit_data = self.get_fit_data_mut(&charge.get_fit_uid());
                    fit_data.sec_zone_active.remove(item_uid);
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                    let item_axt = module.get_axt().unwrap();
                    if let Some(item_grp_aid) = module.get_val_active_group_id() {
                        fit_data.mods_max_group_active_all.remove_entry(item_grp_aid, item_uid);
                        if item_axt.max_group_active_limited {
                            fit_data.mods_max_group_active_limited.remove(item_uid);
                        }
                    }
                    match module.get_max_state().unwrap() {
                        RState::Offline => {
                            fit_data.mods_state.get_mut(item_uid).unwrap().state = ModuleState::Online;
                        }
                        RState::Online => {
                            fit_data.mods_state.remove(item_uid);
                        }
                        _ => (),
                    }
                    if item_axt.sec_zone_limitable {
                        fit_data.sec_zone_active.remove(item_uid);
                    }
                    fit_data.mods_active.remove(item_uid);
                }
                _ => (),
            },
            RState::Overload => {
                if let UItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_uid());
                    let item_axt = module.get_axt().unwrap();
                    match module.get_max_state().unwrap() {
                        RState::Offline | RState::Online => {
                            fit_data.mods_state.get_mut(item_uid).unwrap().state = ModuleState::Active;
                        }
                        RState::Active => {
                            fit_data.mods_state.remove(item_uid);
                        }
                        _ => (),
                    }
                    if item_axt.overload_td_lvl.is_some() {
                        fit_data.overload_td_lvl.remove(item_uid);
                    }
                }
            }
            _ => (),
        }
    }
}
