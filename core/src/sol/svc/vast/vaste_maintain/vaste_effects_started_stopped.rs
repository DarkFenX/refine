use crate::{
    ac, ad,
    sol::{svc::vast::Vast, uad::item::Item},
};

impl Vast {
    pub(in crate::sol::svc) fn effects_started(&mut self, item: &Item, a_effects: &[ad::ArcEffect]) {
        match item {
            Item::Module(module) => {
                for a_effect in a_effects {
                    match a_effect.id {
                        ac::effects::ONLINE => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_svcs_online.insert(module.get_item_id());
                        }
                        ac::effects::TURRET_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_turret.insert(module.get_item_id());
                        }
                        ac::effects::LAUNCHER_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_launcher.insert(module.get_item_id());
                        }
                        _ => (),
                    }
                }
            }
            Item::Rig(rig) => {
                for a_effect in a_effects {
                    if a_effect.id == ac::effects::RIG_SLOT {
                        if let Some(val) = rig.get_a_attrs().unwrap().get(&ac::attrs::UPGRADE_COST) {
                            let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                            fit_data.rigs_rigslot_calibration.insert(rig.get_item_id(), *val);
                        }
                    }
                }
            }
            Item::Service(service) => {
                for a_effect in a_effects {
                    if a_effect.id == ac::effects::ONLINE {
                        let fit_data = self.get_fit_data_mut(&service.get_fit_id()).unwrap();
                        fit_data.mods_svcs_online.insert(service.get_item_id());
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn effects_stopped(&mut self, item: &Item, a_effects: &[ad::ArcEffect]) {
        match item {
            Item::Module(module) => {
                for a_effect in a_effects {
                    match a_effect.id {
                        ac::effects::ONLINE => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_svcs_online.remove(&module.get_item_id());
                        }
                        ac::effects::TURRET_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_turret.remove(&module.get_item_id());
                        }
                        ac::effects::LAUNCHER_FITTED => {
                            let fit_data = self.get_fit_data_mut(&module.get_fit_id()).unwrap();
                            fit_data.mods_launcher.remove(&module.get_item_id());
                        }
                        _ => (),
                    }
                }
            }
            Item::Rig(rig) => {
                for a_effect in a_effects {
                    if a_effect.id == ac::effects::RIG_SLOT {
                        let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                        fit_data.rigs_rigslot_calibration.remove(&rig.get_item_id());
                    }
                }
            }
            Item::Service(service) => {
                for a_effect in a_effects {
                    if a_effect.id == ac::effects::ONLINE {
                        let fit_data = self.get_fit_data_mut(&service.get_fit_id()).unwrap();
                        fit_data.mods_svcs_online.remove(&service.get_item_id());
                    }
                }
            }
            _ => (),
        }
    }
}
