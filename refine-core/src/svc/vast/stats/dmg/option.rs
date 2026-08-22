use crate::{DefOption, nd::NEffectDmgKind, rd::REffect, ud::UItem};

/// Items which will be included in damage stats.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatDmgItemKinds {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::kind_default"))]
    default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    turret: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    missile: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    breacher: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    vorton: DefOption = DefOption::Default,
    /// LR fighter bomb damage is triggered by this flag as well
    #[cfg_attr(feature = "serde", serde(default))]
    bomb: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    smartbomb: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    superweapon: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    minion_mobile: DefOption = DefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    minion_static: DefOption = DefOption::Default,
}
const impl Default for StatDmgItemKinds {
    fn default() -> Self {
        Self { .. }
    }
}
impl StatDmgItemKinds {
    /// True to have all supported item kinds enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    pub fn with_turret(mut self, enabled: bool) -> Self {
        self.turret = enabled.into();
        self
    }
    pub fn with_missile(mut self, enabled: bool) -> Self {
        self.missile = enabled.into();
        self
    }
    pub fn with_breacher(mut self, enabled: bool) -> Self {
        self.breacher = enabled.into();
        self
    }
    pub fn with_vorton(mut self, enabled: bool) -> Self {
        self.vorton = enabled.into();
        self
    }
    /// LR fighter bomb damage is triggered by this flag as well
    pub fn with_bomb(mut self, enabled: bool) -> Self {
        self.bomb = enabled.into();
        self
    }
    pub fn with_smartbomb(mut self, enabled: bool) -> Self {
        self.smartbomb = enabled.into();
        self
    }
    pub fn with_superweapon(mut self, enabled: bool) -> Self {
        self.superweapon = enabled.into();
        self
    }
    pub fn with_minion_mobile(mut self, enabled: bool) -> Self {
        self.minion_mobile = enabled.into();
        self
    }
    pub fn with_minion_static(mut self, enabled: bool) -> Self {
        self.minion_static = enabled.into();
        self
    }
}
impl StatDmgItemKinds {
    pub(in crate::svc::vast::stats) fn resolve(&self, u_item: &UItem, r_effect: &REffect) -> bool {
        match u_item {
            UItem::Drone(drone) => {
                return match drone.get_r_item_attr_data().unwrap().is_mobile {
                    true => self.minion_mobile.is_enabled(self.default),
                    false => self.minion_static.is_enabled(self.default),
                };
            }
            UItem::Fighter(fighter) => {
                return match fighter.get_r_item_attr_data().unwrap().is_mobile {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    pub(super) fn kind_default() -> bool {
        true
    }
}
