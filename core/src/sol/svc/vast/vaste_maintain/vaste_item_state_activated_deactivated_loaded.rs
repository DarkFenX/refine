use crate::{
    ad, consts,
    sol::{
        svc::vast::{SolValModuleStateFail, SolVast},
        uad::item::{SolItem, SolItemState, SolModuleState},
    },
};

impl SolVast {
    pub(in crate::sol::svc) fn item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        match state {
            SolItemState::Online => match item {
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
                SolItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(grp_id, module.get_id());
                        if module
                            .get_attrs()
                            .unwrap()
                            .contains_key(&consts::attrs::MAX_GROUP_ONLINE)
                        {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(module.get_id(), grp_id);
                        }
                    }
                    if let Some(sec_class) = extras.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(module.get_id(), sec_class);
                    }
                    if let ad::AState::Offline = extras.max_state {
                        fit_data.mods_state.insert(
                            module.get_id(),
                            SolValModuleStateFail {
                                item_id: module.get_id(),
                                state: SolModuleState::Online,
                                max_state: SolModuleState::Offline,
                            },
                        );
                    }
                }
                SolItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_id()).unwrap();
                    let extras = service.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .add_entry(grp_id, service.get_id());
                        if service
                            .get_attrs()
                            .unwrap()
                            .contains_key(&consts::attrs::MAX_GROUP_ONLINE)
                        {
                            fit_data
                                .mods_svcs_max_group_online_limited
                                .insert(service.get_id(), grp_id);
                        }
                    }
                    if let Some(sec_class) = extras.online_max_sec_class {
                        fit_data.sec_zone_online_class.insert(service.get_id(), sec_class);
                    }
                }
                _ => (),
            },
            SolItemState::Active => match item {
                SolItem::Charge(charge) => {
                    let extras = charge.get_a_extras().unwrap();
                    if extras.sec_zone_limitable {
                        let fit_data = self.get_fit_data_mut(&charge.get_fit_id()).unwrap();
                        fit_data.sec_zone_active.insert(charge.get_id());
                    }
                }
                SolItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_active_group_id {
                        fit_data.mods_max_group_active_all.add_entry(grp_id, module.get_id());
                        if module
                            .get_attrs()
                            .unwrap()
                            .contains_key(&consts::attrs::MAX_GROUP_ACTIVE)
                        {
                            fit_data.mods_max_group_active_limited.insert(module.get_id(), grp_id);
                        }
                    }
                    match extras.max_state {
                        ad::AState::Offline => {
                            fit_data.mods_state.get_mut(&module.get_id()).unwrap().state = SolModuleState::Active;
                        }
                        ad::AState::Online => {
                            fit_data.mods_state.insert(
                                module.get_id(),
                                SolValModuleStateFail {
                                    item_id: module.get_id(),
                                    state: SolModuleState::Active,
                                    max_state: SolModuleState::Online,
                                },
                            );
                        }
                        _ => (),
                    }
                    if extras.sec_zone_limitable {
                        fit_data.sec_zone_active.insert(module.get_id());
                    }
                    fit_data.mods_active.insert(module.get_id());
                }
                _ => (),
            },
            SolItemState::Overload => {
                if let SolItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    match extras.max_state {
                        ad::AState::Offline | ad::AState::Online => {
                            fit_data.mods_state.get_mut(&module.get_id()).unwrap().state = SolModuleState::Overload;
                        }
                        ad::AState::Active => {
                            fit_data.mods_state.insert(
                                module.get_id(),
                                SolValModuleStateFail {
                                    item_id: module.get_id(),
                                    state: SolModuleState::Overload,
                                    max_state: SolModuleState::Active,
                                },
                            );
                        }
                        _ => (),
                    }
                    if let Some(td_lvl) = extras.overload_td_lvl {
                        fit_data.overload_td_lvl.insert(module.get_id(), td_lvl);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        match state {
            SolItemState::Online => match item {
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
                SolItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(&grp_id, &module.get_id());
                        fit_data.mods_svcs_max_group_online_limited.remove(&module.get_id());
                    }
                    if extras.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(&module.get_id());
                    }
                    if let ad::AState::Offline = extras.max_state {
                        fit_data.mods_state.remove(&module.get_id());
                    }
                }
                SolItem::Service(service) => {
                    let fit_data = self.get_fit_data_mut(&service.get_fit_id()).unwrap();
                    let extras = service.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_online_group_id {
                        fit_data
                            .mods_svcs_max_group_online_all
                            .remove_entry(&grp_id, &service.get_id());
                        fit_data.mods_svcs_max_group_online_limited.remove(&service.get_id());
                    }
                    if extras.online_max_sec_class.is_some() {
                        fit_data.sec_zone_online_class.remove(&service.get_id());
                    }
                }
                _ => (),
            },
            SolItemState::Active => match item {
                SolItem::Charge(charge) => {
                    let fit_data = self.get_fit_data_mut(&charge.get_fit_id()).unwrap();
                    fit_data.sec_zone_active.remove(&charge.get_id());
                }
                SolItem::Module(module) => {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    if let Some(grp_id) = extras.val_active_group_id {
                        fit_data
                            .mods_max_group_active_all
                            .remove_entry(&grp_id, &module.get_id());
                        fit_data.mods_max_group_active_limited.remove(&module.get_id());
                    }
                    match extras.max_state {
                        ad::AState::Offline => {
                            fit_data.mods_state.get_mut(&module.get_id()).unwrap().state = SolModuleState::Online;
                        }
                        ad::AState::Online => {
                            fit_data.mods_state.remove(&module.get_id());
                        }
                        _ => (),
                    }
                    if extras.sec_zone_limitable {
                        fit_data.sec_zone_active.remove(&module.get_id());
                    }
                    fit_data.mods_active.remove(&module.get_id());
                }
                _ => (),
            },
            SolItemState::Overload => {
                if let SolItem::Module(module) = item {
                    let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                    let extras = module.get_a_extras().unwrap();
                    match extras.max_state {
                        ad::AState::Offline | ad::AState::Online => {
                            fit_data.mods_state.get_mut(&module.get_id()).unwrap().state = SolModuleState::Active;
                        }
                        ad::AState::Active => {
                            fit_data.mods_state.remove(&module.get_id());
                        }
                        _ => (),
                    }
                    if extras.overload_td_lvl.is_some() {
                        fit_data.overload_td_lvl.remove(&module.get_id());
                    }
                }
            }
            _ => (),
        }
    }
}
