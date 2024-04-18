use crate::{
    ad,
    defs::{EAttrId, EBuffId, EEffectId},
    ec,
    ss::{
        item::SsItem,
        svc::{
            svce_calc::modifier::{extend_with_custom_mods, SsAttrMod, SsModType},
            SsSvcs,
        },
        SsView,
    },
    util::StMapSetL1,
};

pub(super) struct GeneratedMods {
    pub(super) all: Vec<SsAttrMod>,
    pub(super) dependent_buffs: StMapSetL1<EAttrId, SsAttrMod>,
}
impl GeneratedMods {
    fn new() -> Self {
        Self {
            all: Vec::new(),
            dependent_buffs: StMapSetL1::new(),
        }
    }
}

impl SsSvcs {
    pub(super) fn calc_generate_mods_for_effects(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) -> GeneratedMods {
        let item_id = item.get_id();
        let mut mods = GeneratedMods::new();
        for effect in effects.iter() {
            let mod_type = match get_mod_type(item, effect) {
                Some(mod_type) => mod_type,
                None => continue,
            };
            // Buffs
            if let Some(buff_info) = effect.buff.as_ref() {
                match buff_info.data_source {
                    // TODO: implement buffs with hardcoded values (e.g. disruption lance)
                    ad::AEffectBuffDataSrc::Hardcoded(buff_id, buff_val) => continue,
                    ad::AEffectBuffDataSrc::DefaultAttrs => {
                        for (buff_type_attr_id, buff_val_attr_id) in ec::attrs::BUFF_ATTRS {
                            if let Ok(buff_id) = self.calc_get_item_attr_val(ss_view, &item_id, &buff_type_attr_id) {
                                let buff_mods = get_buff_mods(
                                    ss_view,
                                    item,
                                    effect,
                                    &(buff_id.extra as EBuffId),
                                    &buff_info.scope,
                                    buff_val_attr_id,
                                    mod_type,
                                );
                                mods.dependent_buffs
                                    .extend_entries(buff_type_attr_id, buff_mods.iter().map(|v| *v));
                                mods.all.extend(buff_mods);
                            }
                        }
                    }
                }
            }
            // Regular modifiers
            mods.all.extend(
                effect
                    .mods
                    .iter()
                    .map(|v| SsAttrMod::from_a_effect(item, effect, v, mod_type)),
            );
            // Custom modifiers
            extend_with_custom_mods(item_id, effect.id, &mut mods.all);
        }
        mods
    }
    pub(super) fn calc_generate_dependent_buff_mods<'a>(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effect_ids: impl Iterator<Item = &'a EEffectId>,
        buff_type_attr_id: &EAttrId,
    ) -> Vec<SsAttrMod> {
        let mut mods = Vec::new();
        let buff_value_attr_id = match *buff_type_attr_id {
            ec::attrs::WARFARE_BUFF1_ID => ec::attrs::WARFARE_BUFF1_VAL,
            ec::attrs::WARFARE_BUFF2_ID => ec::attrs::WARFARE_BUFF2_VAL,
            ec::attrs::WARFARE_BUFF3_ID => ec::attrs::WARFARE_BUFF3_VAL,
            ec::attrs::WARFARE_BUFF4_ID => ec::attrs::WARFARE_BUFF4_VAL,
            _ => return mods,
        };
        let item_id = item.get_id();
        for effect_id in effect_ids {
            let effect = ss_view.src.get_a_effect(effect_id).unwrap();
            let mod_type = match get_mod_type(item, &effect) {
                Some(mod_type) => mod_type,
                None => continue,
            };
            if let Some(buff_info) = effect.buff.as_ref() {
                if matches!(buff_info.data_source, ad::AEffectBuffDataSrc::DefaultAttrs) {
                    if let Ok(buff_id) = self.calc_get_item_attr_val(ss_view, &item_id, &buff_type_attr_id) {
                        let buff_mods = get_buff_mods(
                            ss_view,
                            item,
                            &effect,
                            &(buff_id.extra as EBuffId),
                            &buff_info.scope,
                            buff_value_attr_id,
                            mod_type,
                        );
                        mods.extend(buff_mods);
                    }
                }
            }
        }
        mods
    }
}

fn get_mod_type(item: &SsItem, effect: &ad::AEffect) -> Option<SsModType> {
    let buff_scope = effect.buff.as_ref().map(|v| &v.scope);
    match (item, effect.category, buff_scope) {
        // Lib system-wide effects are EVE system effects and buffs
        (SsItem::SwEffect(_), ec::effcats::SYSTEM, None) => Some(SsModType::SystemWide),
        (SsItem::SwEffect(_), ec::effcats::ACTIVE, Some(ad::AEffectBuffScope::Everything)) => {
            Some(SsModType::SystemWide)
        }
        // Lib fit-wide effects are EVE system effects and buffs
        (SsItem::FwEffect(_), ec::effcats::SYSTEM, None) => Some(SsModType::FitWide),
        (SsItem::FwEffect(_), ec::effcats::ACTIVE, Some(ad::AEffectBuffScope::Everything)) => Some(SsModType::FitWide),
        // Lib projected effects are EVE system effects and buffs
        (SsItem::ProjEffect(_), ec::effcats::SYSTEM, None) => Some(SsModType::Projected),
        (SsItem::ProjEffect(_), ec::effcats::ACTIVE, Some(ad::AEffectBuffScope::Everything)) => {
            Some(SsModType::Projected)
        }
        // Fleet buffs
        (SsItem::Module(_), ec::effcats::ACTIVE, Some(ad::AEffectBuffScope::FleetShips)) => Some(SsModType::Fleet),
        // Local modifications
        (_, ec::effcats::PASSIVE | ec::effcats::ACTIVE | ec::effcats::ONLINE | ec::effcats::OVERLOAD, None) => {
            Some(SsModType::Local)
        }
        _ => None,
    }
}

fn get_buff_mods(
    ss_view: &SsView,
    item: &SsItem,
    effect: &ad::AEffect,
    buff_id: &EBuffId,
    buff_scope: &ad::AEffectBuffScope,
    buff_val_id: EAttrId,
    mod_type: SsModType,
) -> Vec<SsAttrMod> {
    let mut mods = Vec::new();
    if let Some(buff) = ss_view.src.get_a_buff(buff_id) {
        for buff_mod in buff.mods.iter() {
            let ss_mod =
                SsAttrMod::from_a_buff(item, effect, &buff, buff_mod, buff_val_id, mod_type, buff_scope.into());
            mods.push(ss_mod);
        }
    }
    mods
}
