use crate::ud::UItem;

#[derive(Copy, Clone)]
pub(crate) enum NEffectDmgKind {
    Turret,
    Missile,
    Breacher,
    Vorton,
    Bomb,
    Smartbomb,
    Superweapon,
}

#[derive(Copy, Clone)]
pub(crate) enum NEffectDmgKindGetter {
    Turret,
    Missile,
    Breacher,
    Vorton,
    Bomb,
    Smartbomb,
    Superweapon,
    // Variants specific to a single effect
    MissileLaunching,
}
impl NEffectDmgKindGetter {
    pub(crate) fn get(&self, u_item: &UItem) -> NEffectDmgKind {
        match self {
            Self::Turret => NEffectDmgKind::Turret,
            Self::Missile => NEffectDmgKind::Missile,
            Self::Breacher => NEffectDmgKind::Breacher,
            Self::Vorton => NEffectDmgKind::Vorton,
            Self::Bomb => NEffectDmgKind::Bomb,
            Self::Smartbomb => NEffectDmgKind::Smartbomb,
            Self::Superweapon => NEffectDmgKind::Superweapon,
            // Variants specific to a single effect
            Self::MissileLaunching => match u_item.is_guided_bomb() {
                true => NEffectDmgKind::Bomb,
                false => NEffectDmgKind::Missile,
            },
        }
    }
}
