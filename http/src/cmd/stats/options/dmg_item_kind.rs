#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HDmgItemKinds {
    #[serde(default)]
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
impl HDmgItemKinds {
    pub(in crate::cmd) fn to_core(&self) -> rc::stats::StatDmgItemKinds {
        let mut core_options = match self.default {
            true => rc::stats::StatDmgItemKinds::all_enabled(),
            false => rc::stats::StatDmgItemKinds::all_disabled(),
        };
        if let Some(turret) = self.turret {
            core_options.turret = turret;
        }
        if let Some(missile) = self.missile {
            core_options.missile = missile;
        }
        if let Some(breacher) = self.breacher {
            core_options.breacher = breacher;
        }
        if let Some(vorton) = self.vorton {
            core_options.vorton = vorton;
        }
        if let Some(bomb) = self.bomb {
            core_options.bomb = bomb;
        }
        if let Some(smartbomb) = self.smartbomb {
            core_options.smartbomb = smartbomb;
        }
        if let Some(superweapon) = self.superweapon {
            core_options.superweapon = superweapon;
        }
        if let Some(minion_mobile) = self.minion_mobile {
            core_options.minion_mobile = minion_mobile;
        }
        if let Some(minion_static) = self.minion_static {
            core_options.minion_static = minion_static;
        }
        core_options
    }
}
