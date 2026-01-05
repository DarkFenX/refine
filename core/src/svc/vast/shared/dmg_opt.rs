use crate::{nd::NEffectDmgKind, rd::REffect, svc::SvcCtx, ud::UItem};

/// Items which will be included in damage stats.
#[derive(Copy, Clone)]
pub struct StatDmgItemKinds {
    pub turret: bool,
    pub missile: bool,
    pub breacher: bool,
    pub vorton: bool,
    pub bomb: bool,
    pub smartbomb: bool,
    pub superweapon: bool,
    pub minion_mobile: bool,
    pub minion_static: bool,
}
impl StatDmgItemKinds {
    /// Include all item types in damage stats.
    pub fn all_enabled() -> Self {
        Self {
            turret: true,
            missile: true,
            breacher: true,
            vorton: true,
            bomb: true,
            smartbomb: true,
            superweapon: true,
            minion_mobile: true,
            minion_static: true,
        }
    }
    /// Exclude all item types from damage stats.
    pub fn all_disabled() -> Self {
        Self {
            turret: false,
            missile: false,
            breacher: false,
            vorton: false,
            bomb: false,
            smartbomb: false,
            superweapon: false,
            minion_mobile: false,
            minion_static: false,
        }
    }
    pub(in crate::svc::vast) fn resolve(&self, ctx: SvcCtx, u_item: &UItem, r_effect: &REffect) -> bool {
        match u_item {
            // Here we assume that autocharges always belong to fighters, and fighters are always
            // mobile
            UItem::Autocharge(autocharge) => {
                let cont_u_item = ctx.u_data.items.get(autocharge.get_cont_item_uid());
                return self.resolve(ctx, cont_u_item, r_effect);
            }
            UItem::Drone(drone) => {
                return match drone.get_axt().unwrap().is_mobile {
                    true => self.minion_mobile,
                    false => self.minion_static,
                };
            }
            UItem::Fighter(fighter) => {
                return match fighter.get_axt().unwrap().is_mobile {
                    true => self.minion_mobile,
                    false => self.minion_static,
                };
            }
            _ => (),
        };
        let dmg_kind = match r_effect.dmg_kind_getter {
            Some(dmg_kind_getter) => dmg_kind_getter(u_item),
            None => return false,
        };
        match dmg_kind {
            NEffectDmgKind::Turret => self.turret,
            NEffectDmgKind::Missile => self.missile,
            NEffectDmgKind::Breacher => self.breacher,
            NEffectDmgKind::Vorton => self.vorton,
            NEffectDmgKind::Bomb => self.bomb,
            NEffectDmgKind::Smartbomb => self.smartbomb,
            NEffectDmgKind::Superweapon => self.superweapon,
        }
    }
}
