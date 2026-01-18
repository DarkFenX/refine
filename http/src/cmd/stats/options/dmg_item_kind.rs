use serde::Deserialize;

use crate::util::default_true;

#[derive(Copy, Clone, educe::Educe, Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatDmgItemKinds {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    turret: Option<bool>,
    missile: Option<bool>,
    breacher: Option<bool>,
    vorton: Option<bool>,
    bomb: Option<bool>,
    smartbomb: Option<bool>,
    superweapon: Option<bool>,
    minion_mobile: Option<bool>,
    minion_static: Option<bool>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatDmgItemKinds {
    pub(in crate::cmd::stats) fn into_core(self) -> rc::stats::StatDmgItemKinds {
        let mut core_item_kinds = match self.default {
            true => rc::stats::StatDmgItemKinds::all_enabled(),
            false => rc::stats::StatDmgItemKinds::all_disabled(),
        };
        if let Some(turret) = self.turret {
            core_item_kinds.turret = turret;
        }
        if let Some(missile) = self.missile {
            core_item_kinds.missile = missile;
        }
        if let Some(breacher) = self.breacher {
            core_item_kinds.breacher = breacher;
        }
        if let Some(vorton) = self.vorton {
            core_item_kinds.vorton = vorton;
        }
        if let Some(bomb) = self.bomb {
            core_item_kinds.bomb = bomb;
        }
        if let Some(smartbomb) = self.smartbomb {
            core_item_kinds.smartbomb = smartbomb;
        }
        if let Some(superweapon) = self.superweapon {
            core_item_kinds.superweapon = superweapon;
        }
        if let Some(minion_mobile) = self.minion_mobile {
            core_item_kinds.minion_mobile = minion_mobile;
        }
        if let Some(minion_static) = self.minion_static {
            core_item_kinds.minion_static = minion_static;
        }
        core_item_kinds
    }
}
