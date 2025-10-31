use crate::util::default_true;

#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
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
impl From<&HStatDmgItemKinds> for rc::stats::StatDmgItemKinds {
    fn from(h_item_kinds: &HStatDmgItemKinds) -> Self {
        let mut core_item_kinds = match h_item_kinds.default {
            true => rc::stats::StatDmgItemKinds::all_enabled(),
            false => rc::stats::StatDmgItemKinds::all_disabled(),
        };
        if let Some(turret) = h_item_kinds.turret {
            core_item_kinds.turret = turret;
        }
        if let Some(missile) = h_item_kinds.missile {
            core_item_kinds.missile = missile;
        }
        if let Some(breacher) = h_item_kinds.breacher {
            core_item_kinds.breacher = breacher;
        }
        if let Some(vorton) = h_item_kinds.vorton {
            core_item_kinds.vorton = vorton;
        }
        if let Some(bomb) = h_item_kinds.bomb {
            core_item_kinds.bomb = bomb;
        }
        if let Some(smartbomb) = h_item_kinds.smartbomb {
            core_item_kinds.smartbomb = smartbomb;
        }
        if let Some(superweapon) = h_item_kinds.superweapon {
            core_item_kinds.superweapon = superweapon;
        }
        if let Some(minion_mobile) = h_item_kinds.minion_mobile {
            core_item_kinds.minion_mobile = minion_mobile;
        }
        if let Some(minion_static) = h_item_kinds.minion_static {
            core_item_kinds.minion_static = minion_static;
        }
        core_item_kinds
    }
}
