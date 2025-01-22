use crate::{
    ad,
    sol::{
        svc::vast::SolVast,
        uad::item::{SolItem, SolItemState},
    },
};

impl SolVast {
    pub(in crate::sol::svc) fn item_state_activated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        if let SolItemState::Online = state {
            if let SolItem::Fighter(fighter) = item {
                if let Some(ad::AItemKind::FighterSquad(fighter_kind)) = fighter.get_a_extras().unwrap().kind {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    match fighter_kind {
                        ad::AFighterKind::Support => fit_data.support_fighters_online.insert(fighter.get_id()),
                        ad::AFighterKind::Light => fit_data.light_fighters_online.insert(fighter.get_id()),
                        ad::AFighterKind::Heavy => fit_data.heavy_fighters_online.insert(fighter.get_id()),
                        ad::AFighterKind::StandupSupport => {
                            fit_data.standup_support_fighters_online.insert(fighter.get_id())
                        }
                        ad::AFighterKind::StandupLight => {
                            fit_data.standup_light_fighters_online.insert(fighter.get_id())
                        }
                        ad::AFighterKind::StandupHeavy => {
                            fit_data.standup_heavy_fighters_online.insert(fighter.get_id())
                        }
                    };
                }
            }
        }
    }
    pub(in crate::sol::svc) fn item_state_deactivated_loaded(&mut self, item: &SolItem, state: &SolItemState) {
        if let SolItemState::Online = state {
            if let SolItem::Fighter(fighter) = item {
                if let Some(ad::AItemKind::FighterSquad(fighter_kind)) = fighter.get_a_extras().unwrap().kind {
                    let fit_data = self.get_fit_data_mut(&fighter.get_fit_id()).unwrap();
                    match fighter_kind {
                        ad::AFighterKind::Support => fit_data.support_fighters_online.remove(&fighter.get_id()),
                        ad::AFighterKind::Light => fit_data.light_fighters_online.remove(&fighter.get_id()),
                        ad::AFighterKind::Heavy => fit_data.heavy_fighters_online.remove(&fighter.get_id()),
                        ad::AFighterKind::StandupSupport => {
                            fit_data.standup_support_fighters_online.remove(&fighter.get_id())
                        }
                        ad::AFighterKind::StandupLight => {
                            fit_data.standup_light_fighters_online.remove(&fighter.get_id())
                        }
                        ad::AFighterKind::StandupHeavy => {
                            fit_data.standup_heavy_fighters_online.remove(&fighter.get_id())
                        }
                    };
                }
            }
        }
    }
}
