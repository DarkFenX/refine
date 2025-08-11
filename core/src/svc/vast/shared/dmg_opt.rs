/// Items which will be included in damage stats.
#[derive(Clone)]
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
}
