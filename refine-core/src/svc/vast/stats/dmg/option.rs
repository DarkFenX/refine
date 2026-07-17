use crate::{misc::DefOption, nd::NEffectDmgKind, rd::REffect, ud::UItem};

/// Items which will be included in damage stats.
#[derive(Copy, Clone)]
pub struct StatDmgItemKinds {
    pub default: bool = true,
    pub turret: DefOption = DefOption::Default,
    pub missile: DefOption = DefOption::Default,
    pub breacher: DefOption = DefOption::Default,
    pub vorton: DefOption = DefOption::Default,
    /// LR fighter bomb damage is triggered by this flag as well
    pub bomb: DefOption = DefOption::Default,
    pub smartbomb: DefOption = DefOption::Default,
    pub superweapon: DefOption = DefOption::Default,
    pub minion_mobile: DefOption = DefOption::Default,
    pub minion_static: DefOption = DefOption::Default,
}
impl StatDmgItemKinds {
    pub(in crate::svc::vast) fn resolve(&self, u_item: &UItem, r_effect: &REffect) -> bool {
        match u_item {
            UItem::Drone(drone) => {
                return match drone.get_axt().unwrap().is_mobile {
                    true => self.minion_mobile.is_enabled(self.default),
                    false => self.minion_static.is_enabled(self.default),
                };
            }
            UItem::Fighter(fighter) => {
                return match fighter.get_axt().unwrap().is_mobile {
                    true => self.minion_mobile.is_enabled(self.default),
                    false => self.minion_static.is_enabled(self.default),
                };
            }
            _ => (),
        };
        let dmg_kind = match r_effect.dmg_kind {
            Some(dmg_kind_getter) => dmg_kind_getter.get(u_item),
            None => return false,
        };
        match dmg_kind {
            NEffectDmgKind::Turret => self.turret.is_enabled(self.default),
            NEffectDmgKind::Missile => self.missile.is_enabled(self.default),
            NEffectDmgKind::Breacher => self.breacher.is_enabled(self.default),
            NEffectDmgKind::Vorton => self.vorton.is_enabled(self.default),
            NEffectDmgKind::Bomb => self.bomb.is_enabled(self.default),
            NEffectDmgKind::Smartbomb => self.smartbomb.is_enabled(self.default),
            NEffectDmgKind::Superweapon => self.superweapon.is_enabled(self.default),
        }
    }
}
