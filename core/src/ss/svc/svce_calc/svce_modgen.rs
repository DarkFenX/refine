use crate::{
    ad,
    defs::{EBuffId},
    ec,
    ss::{
        item::SsItem,
        svc::{
            svce_calc::modifier::{extend_with_custom_mods, SsAttrMod, SsModType},
            SsSvcs,
        },
        SsView,
    },
    EAttrId,
};

impl SsSvcs {
    pub(super) fn calc_generate_mods(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) -> Vec<SsAttrMod> {
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
            // Buffs
            if let Some(buff_info) = effect.buff.as_ref() {
                match buff_info.data_source {
                    // TODO: implement buffs with hardcoded values (e.g. disruption lance)
                    ad::AEffectBuffDataSrc::Hardcoded(buff_id, buff_val) => continue,
                    ad::AEffectBuffDataSrc::DefaultAttrs => {
                        for (buff_attr_id, buff_val_id) in ec::attrs::BUFF_ATTRS {
                            if let Ok(buff_id) = self.calc_get_item_attr_val(ss_view, &item_id, &buff_attr_id) {
                                insert_buff_mods(
                                    &mut mods,
                                    ss_view,
                                    item,
                                    effect,
                                    &(buff_id.extra as EBuffId),
                                    &buff_info.scope,
                                    buff_val_id,
                                    mod_type,
                                );
                            }
                        }
                    }
                }
            }
            // Regular modifiers
            mods.extend(
                effect
                    .mods
                    .iter()
                    .map(|v| SsAttrMod::from_a_effect(item, effect, v, mod_type)),
            );
            // Custom modifiers
            extend_with_custom_mods(item_id, effect.id, &mut mods);
        }
        mods
    }
}

fn insert_buff_mods(
    mods: &mut Vec<SsAttrMod>,
    ss_view: &SsView,
    item: &SsItem,
    effect: &ad::ArcEffect,
    buff_id: &EBuffId,
    buff_scope: &ad::AEffectBuffScope,
    buff_val_id: EAttrId,
    mod_type: SsModType,
) {
    if let Some(buff) = ss_view.src.get_a_buff(buff_id) {
        for buff_mod in buff.mods.iter() {
            let ss_mod =
                SsAttrMod::from_a_buff(item, effect, &buff, buff_mod, buff_val_id, mod_type, buff_scope.into());
            mods.push(ss_mod);
        }
    }
}
