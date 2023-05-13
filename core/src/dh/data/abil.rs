use crate::{defines::ReeInt, util::Named};

/// Fighter ability data.
#[derive(Debug)]
pub struct FighterAbil {
    /// Fighter ability ID.
    pub id: ReeInt,
    /// Fighter ability target mode name.
    pub target_mode: String,
    /// Defines if the ability can be used in hisec.
    pub disallow_hisec: bool,
    /// Defines if the ability can be used in lowsec.
    pub disallow_lowsec: bool,
}
impl FighterAbil {
    /// Make a new fighter ability out of passed data.
    pub fn new<T: Into<String>>(id: ReeInt, target_mode: T, disallow_hisec: bool, disallow_lowsec: bool) -> Self {
        Self {
            id,
            target_mode: target_mode.into(),
            disallow_hisec,
            disallow_lowsec,
        }
    }
}
impl Named for FighterAbil {
    fn get_name() -> &'static str {
        "dh::FighterAbil"
    }
}
