use crate::{
    ac, ad,
    sol::{
        ItemKey, ModuleState,
        svc::vast::{ValModuleStateModuleInfo, Vast},
        uad::item::UadItem,
    },
};

impl Vast {
    pub(in crate::sol::svc) fn item_state_activated_loaded(
        &mut self,
        item_key: ItemKey,
        item: &UadItem,
        a_state: &ad::AState,
    ) {
        match a_state {
            ad::AState::Offline => {
                if let UadItem::Rig(rig) = item
                    && let Some(val) = rig.get_a_attrs().unwrap().get(&ac::attrs::UPGRADE_COST)
                {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_key());
                    fit_data.rigs_offline_calibration.insert(item_key, *val);
                }
            }
            ad::AState::Online => match item {
                UadItem::Fighter(fighter) => {
                    let extras = fighter.get_a_extras().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_key());
                    if extras.is_light_fighter {
                        fit_data.light_fighters_online.insert(item_key);
                    }
                    if extras.is_heavy_fighter {
                        fit_data.heavy_fighters_online.insert(item_key);
                    }
                    if extras.is_support_fighter {
                        fit_data.support_fighters_online.insert(item_key);
                    }
                    if extras.is_standup_light_fighter {
                        fit_data.standup_light_fighters_online.insert(item_key);
                    }
                    if extras.is_standup_heavy_fighter {
                        fit_data.standup_heavy_fighters_online.insert(item_key);
                    }
                    if extras.is_standup_support_fighter {
                        fit_data.standup_support_fighters_online.insert(item_key);
                    }
                }
                UadItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let extras = module.get_a_extras().unwrap();
                    fit_data.mods_svcs_online.insert(item_key);
                    if let Some(a_item_grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(a_item_grp_id, item_key);
                        if module.get_a_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_ONLINE) {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(item_key, a_item_grp_id);
                        }
                    }
                    if let Some(sec_class) = extras.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(item_key, sec_class);
                    }
                    if let ad::AState::Offline = extras.max_state {
                        fit_data.mods_state.insert(
                            item_key,
                            ValModuleStateModuleInfo {
                                state: ModuleState::Online,
                                max_state: ModuleState::Offline,
                            },
                        );
                    }
                }
                UadItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_key());
                    let extras = service.get_a_extras().unwrap();
                    fit_data.mods_svcs_online.insert(item_key);
                    if let Some(a_item_grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(a_item_grp_id, item_key);
                        if service
                            .get_a_attrs()
                            .unwrap()
                            .contains_key(&ac::attrs::MAX_GROUP_ONLINE)
                        {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(item_key, a_item_grp_id);
                        }
                    }
                    if let Some(sec_class) = extras.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(item_key, sec_class);
                    }
                }
                _ => (),
            },
            ad::AState::Active => match item {
                UadItem::Charge(charge) => {
                    let extras = charge.get_a_extras().unwrap();
                    if extras.sec_zone_limitable {
                        let fit_data = self.get_fit_data_mut(&charge.get_fit_key());
                        fit_data.sec_zone_active.insert(item_key);
                    }
                }
                UadItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let extras = module.get_a_extras().unwrap();
                    if let Some(a_item_grp_id) = extras.val_active_group_id {
                        fit_data.mods_max_group_active_all.add_entry(a_item_grp_id, item_key);
                        if module.get_a_attrs().unwrap().contains_key(&ac::attrs::MAX_GROUP_ACTIVE) {
                            fit_data.mods_max_group_active_limited.insert(item_key, a_item_grp_id);
                        }
                    }
                    match extras.max_state {
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
                    if extras.sec_zone_limitable {
                        fit_data.sec_zone_active.insert(item_key);
                    }
                    fit_data.mods_active.insert(item_key);
                }
                _ => (),
            },
            ad::AState::Overload => {
                if let UadItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let extras = module.get_a_extras().unwrap();
                    match extras.max_state {
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
                    if let Some(td_lvl) = extras.overload_td_lvl {
                        fit_data.overload_td_lvl.insert(item_key, td_lvl);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn item_state_deactivated_loaded(
        &mut self,
        item_key: &ItemKey,
        item: &UadItem,
        a_state: &ad::AState,
    ) {
        match a_state {
            ad::AState::Offline => {
                if let UadItem::Rig(rig) = item {
                    let fit_data = self.get_fit_data_mut(&rig.get_fit_key());
                    fit_data.rigs_offline_calibration.remove(item_key);
                }
            }
            ad::AState::Online => match item {
                UadItem::Fighter(fighter) => {
                    let extras = fighter.get_a_extras().unwrap();
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_key());
                    if extras.is_light_fighter {
                        fit_data.light_fighters_online.remove(item_key);
                    }
                    if extras.is_heavy_fighter {
                        fit_data.heavy_fighters_online.remove(item_key);
                    }
                    if extras.is_support_fighter {
                        fit_data.support_fighters_online.remove(item_key);
                    }
                    if extras.is_standup_light_fighter {
                        fit_data.standup_light_fighters_online.remove(item_key);
                    }
                    if extras.is_standup_heavy_fighter {
                        fit_data.standup_heavy_fighters_online.remove(item_key);
                    }
                    if extras.is_standup_support_fighter {
                        fit_data.standup_support_fighters_online.remove(item_key);
                    }
                }
                UadItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let extras = module.get_a_extras().unwrap();
                    fit_data.mods_svcs_online.remove(item_key);
                    if let Some(a_item_grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(&a_item_grp_id, item_key);
                        fit_data.mods_svcs_max_group_online_limited.remove(item_key);
                    }
                    if extras.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(item_key);
                    }
                    if let ad::AState::Offline = extras.max_state {
                        fit_data.mods_state.remove(item_key);
                    }
                }
                UadItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_key());
                    let extras = service.get_a_extras().unwrap();
                    fit_data.mods_svcs_online.remove(item_key);
                    if let Some(a_item_grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(&a_item_grp_id, item_key);
                        fit_data.mods_svcs_max_group_online_limited.remove(item_key);
                    }
                    if extras.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(item_key);
                    }
                }
                _ => (),
            },
            ad::AState::Active => match item {
                UadItem::Charge(charge) => {
                    let fit_data = self.get_fit_data_mut(&charge.get_fit_key());
                    fit_data.sec_zone_active.remove(item_key);
                }
                UadItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let extras = module.get_a_extras().unwrap();
                    if let Some(a_item_grp_id) = extras.val_active_group_id {
                        fit_data
                            .mods_max_group_active_all
                            .remove_entry(&a_item_grp_id, item_key);
                        fit_data.mods_max_group_active_limited.remove(item_key);
                    }
                    match extras.max_state {
                        ad::AState::Offline => {
                            fit_data.mods_state.get_mut(item_key).unwrap().state = ModuleState::Online;
                        }
                        ad::AState::Online => {
                            fit_data.mods_state.remove(item_key);
                        }
                        _ => (),
                    }
                    if extras.sec_zone_limitable {
                        fit_data.sec_zone_active.remove(item_key);
                    }
                    fit_data.mods_active.remove(item_key);
                }
                _ => (),
            },
            ad::AState::Overload => {
                if let UadItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_key());
                    let extras = module.get_a_extras().unwrap();
                    match extras.max_state {
                        ad::AState::Offline | ad::AState::Online => {
                            fit_data.mods_state.get_mut(item_key).unwrap().state = ModuleState::Active;
                        }
                        ad::AState::Active => {
                            fit_data.mods_state.remove(item_key);
                        }
                        _ => (),
                    }
                    if extras.overload_td_lvl.is_some() {
                        fit_data.overload_td_lvl.remove(item_key);
                    }
                }
            }
            _ => (),
        }
    }
}
