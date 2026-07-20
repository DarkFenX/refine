use crate::{misc::DefOption, nd::NEffectDmgKind, rd::REffect, ud::UItem};

/// Items which will be included in damage stats.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatDmgItemKinds {
    #[cfg_attr(feature = "serde", serde(default))]
    pub default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    pub turret: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub missile: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub breacher: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub vorton: DefOption = DefOption::Default,
    /// LR fighter bomb damage is triggered by this flag as well
    #[cfg_attr(feature = "serde", serde(default))]
    pub bomb: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub smartbomb: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub superweapon: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub minion_mobile: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
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
