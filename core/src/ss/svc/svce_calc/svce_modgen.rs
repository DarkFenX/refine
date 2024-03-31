use crate::{
    ad, ec,
    ss::{
        item::SsItem,
        svc::{
            svce_calc::modifier::{extend_with_custom_mods, SsAttrMod, SsModType},
            SsSvcs,
        },
    },
};

impl SsSvcs {
    pub(super) fn calc_generate_mods(&mut self, item: &SsItem, effects: &Vec<ad::ArcEffect>) -> Vec<SsAttrMod> {
        let item_id = item.get_id();
        let mut mods = Vec::new();
        for effect in effects.iter() {
            let buff_scope = effect.buff.as_ref().map(|v| &v.scope);
            let mod_type = match (item, effect.category, buff_scope) {
                // Lib system-wide effects are EVE system effects and buffs
                (SsItem::SwEffect(_), ec::effcats::SYSTEM, None) => SsModType::SystemWide,
                (SsItem::SwEffect(_), ec::effcats::ACTIVE, Some(ad::AEffectBuffScope::Everything)) => {
                    SsModType::SystemWide
                }
                // Lib fit-wide effects are EVE system effects and buffs
                (SsItem::FwEffect(_), ec::effcats::SYSTEM, None) => SsModType::FitWide,
                (SsItem::FwEffect(_), ec::effcats::ACTIVE, Some(ad::AEffectBuffScope::Everything)) => {
                    SsModType::FitWide
                }
                // Lib projected effects are EVE system effects and buffs
                (SsItem::ProjEffect(_), ec::effcats::SYSTEM, None) => SsModType::Projected,
                (SsItem::ProjEffect(_), ec::effcats::ACTIVE, Some(ad::AEffectBuffScope::Everything)) => {
                    SsModType::Projected
                }
                // Fleet buffs
                (SsItem::Module(_), ec::effcats::ACTIVE, Some(ad::AEffectBuffScope::FleetShips)) => SsModType::Fleet,
                // Local modifications
                (_, ec::effcats::PASSIVE | ec::effcats::ACTIVE | ec::effcats::ONLINE | ec::effcats::OVERLOAD, None) => {
                    SsModType::Local
                }
                _ => continue,
            };
            // Regular modifiers
            mods.extend(
                effect
                    .mods
                    .iter()
                    .map(|v| SsAttrMod::from_a_data(item, effect, v, mod_type)),
            );
            // Custom modifiers
            extend_with_custom_mods(item_id, effect.id, &mut mods);
        }
        mods
    }
}
