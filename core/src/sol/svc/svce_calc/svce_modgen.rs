use crate::{
    ad,
    defs::{EAttrId, EBuffId, EEffectId},
    ec,
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{extend_with_custom_mods, SolRawModifier},
            SolSvcs,
        },
        SolView,
    },
};

impl SolSvcs {
    pub(super) fn calc_generate_mods_for_effect(
        &mut self,
        modifiers: &mut Vec<SolRawModifier>,
        sol_view: &SolView,
        item: &SolItem,
        effect: &ad::AEffect,
    ) {
        modifiers.clear();
        let item_id = item.get_id();
        // Regular modifiers
        for a_mod in effect.mods.iter() {
            match SolRawModifier::from_a_modifier(item, effect, a_mod) {
                Some(sol_mod) => modifiers.push(sol_mod),
                None => continue,
            };
        }
        // Buffs
        if let Some(buff_info) = effect.buff.as_ref() {
            match &buff_info.source {
                ad::AEffectBuffSrc::DefaultAttrs => {
                    for (buff_type_attr_id, buff_val_attr_id) in ec::extras::BUFF_STDATTRS {
                        if let Ok(buff_id) = self.calc_get_item_attr_val(sol_view, &item_id, &buff_type_attr_id) {
                            add_buff_mods(
                                modifiers,
                                sol_view,
                                item,
                                effect,
                                &(buff_id.extra as EBuffId),
                                &buff_info.scope,
                                Some(buff_type_attr_id),
                                buff_val_attr_id,
                            );
                        }
                    }
                }
                ad::AEffectBuffSrc::Customized(buff_custom_srcs) => {
                    for buff_custom_src in buff_custom_srcs {
                        match buff_custom_src {
                            ad::AEffectBuffSrcCustom::AffectorVal(buff_id, buff_val_attr_id) => add_buff_mods(
                                modifiers,
                                sol_view,
                                item,
                                effect,
                                &buff_id,
                                &buff_info.scope,
                                None,
                                *buff_val_attr_id,
                            ),
                            // TODO: implement buffs with hardcoded values (e.g. disruption lance)
                            ad::AEffectBuffSrcCustom::HardcodedVal(_, _) => (),
                        }
                    }
                }
            }
        }
        // Custom modifiers
        extend_with_custom_mods(item_id, effect.id, modifiers);
    }
    pub(super) fn calc_generate_dependent_buff_mods<'a>(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effect_ids: impl Iterator<Item = &'a EEffectId>,
        buff_type_attr_id: &EAttrId,
    ) -> Vec<SolRawModifier> {
        let mut modifiers = Vec::new();
        let buff_value_attr_id = match *buff_type_attr_id {
            ec::attrs::WARFARE_BUFF1_ID => ec::attrs::WARFARE_BUFF1_VAL,
            ec::attrs::WARFARE_BUFF2_ID => ec::attrs::WARFARE_BUFF2_VAL,
            ec::attrs::WARFARE_BUFF3_ID => ec::attrs::WARFARE_BUFF3_VAL,
            ec::attrs::WARFARE_BUFF4_ID => ec::attrs::WARFARE_BUFF4_VAL,
            _ => return modifiers,
        };
        let item_id = item.get_id();
        for effect_id in effect_ids {
            let effect = sol_view.src.get_a_effect(effect_id).unwrap();
            if let Some(buff_info) = effect.buff.as_ref() {
                if matches!(buff_info.source, ad::AEffectBuffSrc::DefaultAttrs) {
                    if let Ok(buff_id) = self.calc_get_item_attr_val(sol_view, &item_id, &buff_type_attr_id) {
                        add_buff_mods(
                            &mut modifiers,
                            sol_view,
                            item,
                            effect,
                            &(buff_id.extra as EBuffId),
                            &buff_info.scope,
                            Some(*buff_type_attr_id),
                            buff_value_attr_id,
                        );
                    }
                }
            }
        }
        modifiers
    }
}

fn add_buff_mods(
    modifiers: &mut Vec<SolRawModifier>,
    sol_view: &SolView,
    item: &SolItem,
    effect: &ad::AEffect,
    buff_id: &EBuffId,
    buff_scope: &ad::AEffectBuffScope,
    buff_type_attr_id: Option<EAttrId>,
    buff_val_attr_id: EAttrId,
) {
    if let Some(buff) = sol_view.src.get_a_buff(buff_id) {
        for buff_mod in buff.mods.iter() {
            let modifier = match SolRawModifier::from_a_buff(
                item,
                effect,
                &buff,
                buff_mod,
                buff_val_attr_id,
                buff_scope.into(),
                buff_type_attr_id,
            ) {
                Some(modifier) => modifier,
                None => continue,
            };
            modifiers.push(modifier);
        }
    }
}
