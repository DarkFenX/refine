use crate::{defs::EAbilId, util::Named};

/// EVE fighter ability data.
pub struct EFighterAbil {
    /// Fighter ability ID.
    pub id: EAbilId,
    /// Defines if the ability can be used in hisec.
    pub disallow_hisec: bool,
    /// Defines if the ability can be used in lowsec.
    pub disallow_lowsec: bool,
}
impl EFighterAbil {
    /// Make a new fighter ability out of passed data.
    pub fn new(id: EAbilId, disallow_hisec: bool, disallow_lowsec: bool) -> Self {
        Self {
            id,
            disallow_hisec,
            disallow_lowsec,
        }
    }
}
impl Named for EFighterAbil {
    fn get_name() -> &'static str {
        "EFighterAbil"
    }
}
