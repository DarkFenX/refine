use crate::{
    ModuleState,
    rd::RState,
    svc::{Vast, vast::validators::ValModuleStateModuleStored},
    ud::{UItem, UItemId},
};

impl Vast {
    pub(in crate::svc) fn item_state_activated_loaded(&mut self, item_uid: UItemId, item: &UItem, state: RState) {
        match state {
            RState::Offline => {
                if let UItem::Rig(rig) = item
                    && let Some(val) = rig.get_r_item_attr_data().unwrap().calibration_use
                {
                    let fit_data = self.get_fit_data_mut(rig.get_fit_uid());
                    fit_data.rigs_offline_calibration.insert(item_uid, val);
                }
            }
            RState::Online => match item {
                UItem::Fighter(fighter) => {
                    let fighter_riad = fighter.get_r_item_attr_data().unwrap();
                    let fit_data = self.get_fit_data_mut(fighter.get_fit_uid());
                    if fighter_riad.is_light_fighter {
                        fit_data.light_fighters_online.insert(item_uid);
                    }
                    if fighter_riad.is_heavy_fighter {
                        fit_data.heavy_fighters_online.insert(item_uid);
                    }
                    if fighter_riad.is_support_fighter {
                        fit_data.support_fighters_online.insert(item_uid);
                    }
                    if fighter_riad.is_st_light_fighter {
                        fit_data.st_light_fighters_online.insert(item_uid);
                    }
                    if fighter_riad.is_st_heavy_fighter {
                        fit_data.st_heavy_fighters_online.insert(item_uid);
                    }
                    if fighter_riad.is_st_support_fighter {
                        fit_data.st_support_fighters_online.insert(item_uid);
                    }
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(module.get_fit_uid());
                    let module_rib = module.get_r_item_base().unwrap();
                    let module_riad = module.get_r_item_attr_data().unwrap();
                    fit_data.mods_svcs_online.insert(item_uid);
                    if let Some(item_grp_aid) = module_rib.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(item_grp_aid, item_uid);
                        if module_riad.max_group_online_limited {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(item_uid, item_grp_aid);
                        }
                    }
                    if let Some(sec_class) = module_riad.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(item_uid, sec_class);
                    }
                    if module_riad.enables_conduit {
                        fit_data.conduit_enablers.insert(item_uid);
                    }
                    if module_rib.max_state == RState::Offline {
                        fit_data.mods_state.insert(
                            item_uid,
                            ValModuleStateModuleStored {
                                state: ModuleState::Online,
                                max_state: ModuleState::Offline,
                            },
                        );
                    }
                }
                UItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(service.get_fit_uid());
                    let service_rib = service.get_r_item_base().unwrap();
                    let service_riad = service.get_r_item_attr_data().unwrap();
                    fit_data.mods_svcs_online.insert(item_uid);
                    if let Some(item_grp_aid) = service_rib.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(item_grp_aid, item_uid);
                        if service_riad.max_group_online_limited {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(item_uid, item_grp_aid);
                        }
                    }
                    if let Some(sec_class) = service_riad.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(item_uid, sec_class);
                    }
                    if service_rib.enables_portal {
                        fit_data.portal_enablers.insert(item_uid);
                    }
                }
                _ => (),
            },
            RState::Active => match item {
                UItem::Charge(charge) => {
                    let charge_riad = charge.get_r_item_attr_data().unwrap();
                    if charge_riad.sec_zone_limitable {
                        let fit_data = self.get_fit_data_mut(charge.get_fit_uid());
                        fit_data.sec_zone_active.insert(item_uid);
                    }
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(module.get_fit_uid());
                    let module_rib = module.get_r_item_base().unwrap();
                    let module_riad = module.get_r_item_attr_data().unwrap();
                    if let Some(item_grp_aid) = module_rib.val_active_group_id {
                        fit_data.mods_max_group_active_all.add_entry(item_grp_aid, item_uid);
                        if module_riad.max_group_active_limited {
                            fit_data.mods_max_group_active_limited.insert(item_uid, item_grp_aid);
                        }
                    }
                    match module_rib.max_state {
                        RState::Offline => {
                            fit_data.mods_state.get_mut(&item_uid).unwrap().state = ModuleState::Active;
                        }
                        RState::Online => {
                            fit_data.mods_state.insert(
                                item_uid,
                                ValModuleStateModuleStored {
                                    state: ModuleState::Active,
                                    max_state: ModuleState::Online,
                                },
                            );
                        }
                        _ => (),
                    }
                    if module_riad.sec_zone_limitable {
                        fit_data.sec_zone_active.insert(item_uid);
                    }
                    fit_data.mods_active.insert(item_uid);
                    if module_riad.enables_portal {
                        fit_data.portal_enablers.insert(item_uid);
                    }
                }
                _ => (),
            },
            RState::Overload => {
                if let UItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(module.get_fit_uid());
                    let module_rib = module.get_r_item_base().unwrap();
                    let module_riad = module.get_r_item_attr_data().unwrap();
                    match module_rib.max_state {
                        RState::Offline | RState::Online => {
                            fit_data.mods_state.get_mut(&item_uid).unwrap().state = ModuleState::Overload;
                        }
                        RState::Active => {
                            fit_data.mods_state.insert(
                                item_uid,
                                ValModuleStateModuleStored {
                                    state: ModuleState::Overload,
                                    max_state: ModuleState::Active,
                                },
                            );
                        }
                        _ => (),
                    }
                    if let Some(td_lvl) = module_riad.overload_td_lvl {
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
                    let fit_data = self.get_fit_data_mut(rig.get_fit_uid());
                    fit_data.rigs_offline_calibration.remove(item_uid);
                }
            }
            RState::Online => match item {
                UItem::Fighter(fighter) => {
                    let fighter_riad = fighter.get_r_item_attr_data().unwrap();
                    let fit_data = self.get_fit_data_mut(fighter.get_fit_uid());
                    if fighter_riad.is_light_fighter {
                        fit_data.light_fighters_online.remove(item_uid);
                    }
                    if fighter_riad.is_heavy_fighter {
                        fit_data.heavy_fighters_online.remove(item_uid);
                    }
                    if fighter_riad.is_support_fighter {
                        fit_data.support_fighters_online.remove(item_uid);
                    }
                    if fighter_riad.is_st_light_fighter {
                        fit_data.st_light_fighters_online.remove(item_uid);
                    }
                    if fighter_riad.is_st_heavy_fighter {
                        fit_data.st_heavy_fighters_online.remove(item_uid);
                    }
                    if fighter_riad.is_st_support_fighter {
                        fit_data.st_support_fighters_online.remove(item_uid);
                    }
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(module.get_fit_uid());
                    let module_rib = module.get_r_item_base().unwrap();
                    let module_riad = module.get_r_item_attr_data().unwrap();
                    fit_data.mods_svcs_online.remove(item_uid);
                    if let Some(item_grp_aid) = module_rib.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(item_grp_aid, item_uid);
                        fit_data.mods_svcs_max_group_online_limited.remove(item_uid);
                    }
                    if module_riad.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(item_uid);
                    }
                    if module_riad.enables_conduit {
                        fit_data.conduit_enablers.remove(item_uid);
                    }
                    if module_rib.max_state == RState::Offline {
                        fit_data.mods_state.remove(item_uid);
                    }
                }
                UItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(service.get_fit_uid());
                    let service_rib = service.get_r_item_base().unwrap();
                    let service_riad = service.get_r_item_attr_data().unwrap();
                    fit_data.mods_svcs_online.remove(item_uid);
                    if let Some(item_grp_aid) = service_rib.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(item_grp_aid, item_uid);
                        fit_data.mods_svcs_max_group_online_limited.remove(item_uid);
                    }
                    if service_riad.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(item_uid);
                    }
                    if service_rib.enables_portal {
                        fit_data.portal_enablers.remove(item_uid);
                    }
                }
                _ => (),
            },
            RState::Active => match item {
                UItem::Charge(charge) => {
                    let fit_data = self.get_fit_data_mut(charge.get_fit_uid());
                    fit_data.sec_zone_active.remove(item_uid);
                }
                UItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(module.get_fit_uid());
                    let module_rib = module.get_r_item_base().unwrap();
                    let module_riad = module.get_r_item_attr_data().unwrap();
                    if let Some(item_grp_aid) = module_rib.val_active_group_id {
                        fit_data.mods_max_group_active_all.remove_entry(item_grp_aid, item_uid);
                        if module_riad.max_group_active_limited {
                            fit_data.mods_max_group_active_limited.remove(item_uid);
                        }
                    }
                    match module_rib.max_state {
                        RState::Offline => {
                            fit_data.mods_state.get_mut(item_uid).unwrap().state = ModuleState::Online;
                        }
                        RState::Online => {
                            fit_data.mods_state.remove(item_uid);
                        }
                        _ => (),
                    }
                    if module_riad.sec_zone_limitable {
                        fit_data.sec_zone_active.remove(item_uid);
                    }
                    fit_data.mods_active.remove(item_uid);
                    if module_riad.enables_portal {
                        fit_data.portal_enablers.remove(item_uid);
                    }
                }
                _ => (),
            },
            RState::Overload => {
                if let UItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(module.get_fit_uid());
                    let module_rib = module.get_r_item_base().unwrap();
                    let module_riad = module.get_r_item_attr_data().unwrap();
                    match module_rib.max_state {
                        RState::Offline | RState::Online => {
                            fit_data.mods_state.get_mut(item_uid).unwrap().state = ModuleState::Active;
                        }
                        RState::Active => {
                            fit_data.mods_state.remove(item_uid);
                        }
                        _ => (),
                    }
                    if module_riad.overload_td_lvl.is_some() {
                        fit_data.overload_td_lvl.remove(item_uid);
                    }
                }
            }
            _ => (),
        }
    }
}
