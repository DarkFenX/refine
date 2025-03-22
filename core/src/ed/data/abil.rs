use crate::{ed::EAbilId, util::Named};

/// EVE fighter ability data.
pub struct EFighterAbil {
    /// Fighter ability ID.
    pub id: EAbilId,
    /// Defines if the ability can be used in hisec.
    pub disallow_hisec: bool,
    /// Defines if the ability can be used in lowsec.
    pub disallow_lowsec: bool,
}
impl Named for EFighterAbil {
    fn get_name() -> &'static str {
        "EFighterAbil"
    }
}
