use crate::{
    ad,
    ss::{
        item::SsItem,
        svc::calc::modifier::{extend_with_custom_mods, SsAttrMod},
    },
};

pub(in crate::ss::svc::calc) struct CategorizedMods {
    pub(in crate::ss::svc::calc) local: Vec<SsAttrMod>,
    pub(in crate::ss::svc::calc) system_wide: Vec<SsAttrMod>,
    pub(in crate::ss::svc::calc) projected: Vec<SsAttrMod>,
    pub(in crate::ss::svc::calc) fleet: Vec<SsAttrMod>,
}
impl CategorizedMods {
    fn new() -> Self {
        Self {
            local: Vec::new(),
            system_wide: Vec::new(),
            projected: Vec::new(),
            fleet: Vec::new(),
        }
    }
    pub(in crate::ss::svc::calc) fn from_item_effects(item: &SsItem, effects: &Vec<ad::ArcEffect>) -> Self {
        let item_id = item.get_id();
        let mut mods = Self::new();
        for effect in effects.iter() {
            let vec = if effect.is_system_wide {
                // Notion of "projected" and "system-wise" differs between the adapted data part and
                // the calculator part. In adapted data, projected buff is an effect which applied
                // via the buff system, like abyssal weather, and system-wide modification is an
                // effect with "system" category, which is applied to all the ships in the system.
                // Since the lib hides how modification is made and allows both to be used as both
                // projected effect and system-wide effect,
                if matches!(item, SsItem::SwEffect(_)) {
                    &mut mods.system_wide
                } else {
                    &mut mods.projected
                }
            } else if effect.is_fleet() {
                // Fleet buff means fleet modifiers only
                &mut mods.fleet
            } else if effect.is_targeted() {
                // For now we assume targeted effects have only projected modifiers
                &mut mods.projected
            } else {
                // Untargeted effect means only local modifiers
                &mut mods.local
            };
            // Regular modifiers
            vec.extend(effect.mods.iter().map(|v| SsAttrMod::from_a_data(item, effect, v)));
            // Custom modifiers
            extend_with_custom_mods(item_id, effect.id, vec);
        }
        mods
    }
    pub(in crate::ss::svc::calc) fn iter_all(&self) -> impl Iterator<Item = &SsAttrMod> {
        itertools::chain![&self.local, &self.system_wide, &self.projected, &self.fleet]
    }
}
