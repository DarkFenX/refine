use crate::{consts::TgtMode, defines::ReeInt};

/// Represents a fighter ability.
///
/// Ability struct itself doesn't carry much data, most of ability data is actually defined on
/// item-ability mappings.
#[derive(Debug)]
pub struct FighterAbil {
    /// Fighter ability ID.
    pub id: ReeInt,
    /// Defines what kind of target you need to activate the ability.
    pub tgt_mode: TgtMode,
    /// Defines if the ability can be used in hisec.
    pub hisec: bool,
    /// Defines if the ability can be used in lowsec.
    pub lowsec: bool,
}
impl FighterAbil {
    /// Make a new fighter ability out of passed data.
    pub fn new(id: ReeInt, tgt_mode: TgtMode, hisec: bool, lowsec: bool) -> FighterAbil {
        FighterAbil {
            id,
            tgt_mode,
            hisec,
            lowsec,
        }
    }
}
