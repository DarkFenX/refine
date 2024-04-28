use crate::{
    ad,
    defs::{EAttrId, EBuffId, EEffectId},
    ec,
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{extend_with_custom_mods, SolAttrMod, SolModType},
            SolSvcs,
        },
        SolView,
    },
    util::StMapSetL1,
};

pub(super) struct GeneratedMods {
    pub(super) all: Vec<SolAttrMod>,
    pub(super) dependent_buffs: StMapSetL1<EAttrId, SolAttrMod>,
}
impl GeneratedMods {
    fn new() -> Self {
        Self {
            all: Vec::new(),
            dependent_buffs: StMapSetL1::new(),
        }
    }
}

impl SolSvcs {
    pub(super) fn calc_generate_mods_for_effects(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) -> GeneratedMods {
        let item_id = item.get_id();
        let mut mods = GeneratedMods::new();
        for effect in effects.iter() {
            let mod_type = match get_mod_type(effect) {
                Some(mod_type) => mod_type,
                None => continue,
            };
            // Buffs
            if let Some(buff_info) = effect.buff.as_ref() {
                match buff_info.data_source {
                    ad::AEffectBuffDataSrc::DefaultAttrs => {
                        for (buff_type_attr_id, buff_val_attr_id) in ec::attrs::BUFF_ATTRS {
                            if let Ok(buff_id) = self.calc_get_item_attr_val(sol_view, &item_id, &buff_type_attr_id) {
                                let buff_mods = get_buff_mods(
                                    sol_view,
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
                    // TODO: implement buffs with hardcoded IDs (e.g. remote web bursts)
                    ad::AEffectBuffDataSrc::HardcodedId(_, _) => continue,
                    // TODO: implement buffs with hardcoded values (e.g. disruption lance)
                    ad::AEffectBuffDataSrc::HardcodedAll(_, _) => continue,
                }
            }
            // Regular modifiers
            mods.all.extend(
                effect
                    .mods
                    .iter()
                    .map(|v| SolAttrMod::from_a_effect(item, effect, v, mod_type)),
            );
            // Custom modifiers
            extend_with_custom_mods(item_id, effect.id, &mut mods.all);
        }
        mods
    }
    pub(super) fn calc_generate_dependent_buff_mods<'a>(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effect_ids: impl Iterator<Item = &'a EEffectId>,
        buff_type_attr_id: &EAttrId,
    ) -> Vec<SolAttrMod> {
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
            let effect = sol_view.src.get_a_effect(effect_id).unwrap();
            let mod_type = match get_mod_type(effect) {
                Some(mod_type) => mod_type,
                None => continue,
            };
            if let Some(buff_info) = effect.buff.as_ref() {
                if matches!(buff_info.data_source, ad::AEffectBuffDataSrc::DefaultAttrs) {
                    if let Ok(buff_id) = self.calc_get_item_attr_val(sol_view, &item_id, &buff_type_attr_id) {
                        let buff_mods = get_buff_mods(
                            sol_view,
                            item,
                            effect,
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

fn get_mod_type(effect: &ad::AEffect) -> Option<SolModType> {
    match (effect.category, &effect.buff) {
        // Local modifications
        (ec::effcats::PASSIVE | ec::effcats::ACTIVE | ec::effcats::ONLINE | ec::effcats::OVERLOAD, None) => {
            Some(SolModType::Local)
        }
        // Buffs
        (ec::effcats::ACTIVE, Some(buff_info)) => match buff_info.scope {
            ad::AEffectBuffScope::FleetShips => Some(SolModType::FleetBuff),
            _ => Some(SolModType::Buff),
        },
        // Lib system-wide effects are EVE system effects and buffs
        (ec::effcats::SYSTEM, None) => Some(SolModType::System),
        // Targeted effects
        (ec::effcats::TARGET, None) => Some(SolModType::Targeted),
        _ => None,
    }
}

fn get_buff_mods(
    sol_view: &SolView,
    item: &SolItem,
    effect: &ad::AEffect,
    buff_id: &EBuffId,
    buff_scope: &ad::AEffectBuffScope,
    buff_val_id: EAttrId,
    mod_type: SolModType,
) -> Vec<SolAttrMod> {
    let mut mods = Vec::new();
    if let Some(buff) = sol_view.src.get_a_buff(buff_id) {
        for buff_mod in buff.mods.iter() {
            let modifier =
                SolAttrMod::from_a_buff(item, effect, &buff, buff_mod, buff_val_id, mod_type, buff_scope.into());
            mods.push(modifier);
        }
    }
    mods
}
