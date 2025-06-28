use crate::{
    ac, ad,
    sol::{
        ItemKey,
        svc::{
            EffectSpec, SvcCtx,
            calc::{Calc, RawModifier, extend_with_custom_mods},
        },
        uad::item::UadItem,
    },
};

impl Calc {
    pub(super) fn generate_mods_for_effect(
        &mut self,
        modifiers: &mut Vec<RawModifier>,
        ctx: &SvcCtx,
        item_key: ItemKey,
        item: &UadItem,
        a_effect: &ad::AEffectRt,
    ) {
        modifiers.clear();
        // Regular modifiers
        for a_mod in a_effect.ae.mods.iter() {
            match RawModifier::try_from_a_modifier(item_key, item, a_effect, a_mod) {
                Some(sol_mod) => modifiers.push(sol_mod),
                None => continue,
            };
        }
        // Buffs
        if let Some(a_buff_info) = a_effect.ae.buff.as_ref() {
            match &a_buff_info.source {
                ad::AEffectBuffSrc::DefaultAttrs => {
                    for (buff_type_a_attr_id, buff_val_a_attr_id) in ac::extras::BUFF_STDATTRS {
                        if let Ok(buff_id) = self.get_item_attr_val_full(ctx, item_key, &buff_type_a_attr_id) {
                            add_buff_mods(
                                modifiers,
                                ctx,
                                item_key,
                                item,
                                a_effect,
                                &(buff_id.extra.round() as ad::ABuffId),
                                &a_buff_info.scope,
                                Some(buff_type_a_attr_id),
                                buff_val_a_attr_id,
                            );
                        }
                    }
                }
                ad::AEffectBuffSrc::Customized(a_buff_custom_srcs) => {
                    for a_buff_custom_src in a_buff_custom_srcs {
                        match a_buff_custom_src {
                            ad::AEffectBuffSrcCustom::AffectorVal(a_buff_id, buff_val_a_attr_id) => add_buff_mods(
                                modifiers,
                                ctx,
                                item_key,
                                item,
                                a_effect,
                                a_buff_id,
                                &a_buff_info.scope,
                                None,
                                *buff_val_a_attr_id,
                            ),
                            ad::AEffectBuffSrcCustom::HardcodedVal(a_buff_id, buff_a_val) => add_buff_mods_hardcoded(
                                modifiers,
                                ctx,
                                item_key,
                                item,
                                a_effect,
                                a_buff_id,
                                &a_buff_info.scope,
                                *buff_a_val,
                            ),
                        }
                    }
                }
            }
        }
        // Custom modifiers
        extend_with_custom_mods(EffectSpec::new(item_key, a_effect.ae.id), modifiers);
        if let Some(customizer) = a_effect.rt.calc_custom_fn {
            customizer(modifiers, item_key);
        }
    }
    pub(super) fn generate_dependent_buff_mods<'a>(
        &mut self,
        ctx: &SvcCtx,
        item_key: ItemKey,
        item: &UadItem,
        a_effect_ids: impl Iterator<Item = &'a ad::AEffectId>,
        buff_type_a_attr_id: ad::AAttrId,
    ) -> Vec<RawModifier> {
        let mut modifiers = Vec::new();
        let buff_value_a_attr_id = match buff_type_a_attr_id {
            ac::attrs::WARFARE_BUFF1_ID => ac::attrs::WARFARE_BUFF1_VAL,
            ac::attrs::WARFARE_BUFF2_ID => ac::attrs::WARFARE_BUFF2_VAL,
            ac::attrs::WARFARE_BUFF3_ID => ac::attrs::WARFARE_BUFF3_VAL,
            ac::attrs::WARFARE_BUFF4_ID => ac::attrs::WARFARE_BUFF4_VAL,
            _ => return modifiers,
        };
        for a_effect_id in a_effect_ids {
            let a_effect = ctx.uad.src.get_a_effect(a_effect_id).unwrap();
            if let Some(a_buff_info) = a_effect.ae.buff.as_ref()
                && matches!(a_buff_info.source, ad::AEffectBuffSrc::DefaultAttrs)
                && let Ok(buff_id_cval) = self.get_item_attr_val_full(ctx, item_key, &buff_type_a_attr_id)
            {
                add_buff_mods(
                    &mut modifiers,
                    ctx,
                    item_key,
                    item,
                    a_effect,
                    &(buff_id_cval.extra.round() as ad::ABuffId),
                    &a_buff_info.scope,
                    Some(buff_type_a_attr_id),
                    buff_value_a_attr_id,
                );
            }
        }
        modifiers
    }
}

fn add_buff_mods(
    modifiers: &mut Vec<RawModifier>,
    ctx: &SvcCtx,
    item_key: ItemKey,
    item: &UadItem,
    a_effect: &ad::AEffectRt,
    a_buff_id: &ad::ABuffId,
    a_buff_scope: &ad::AEffectBuffScope,
    buff_type_a_attr_id: Option<ad::AAttrId>,
    buff_val_a_attr_id: ad::AAttrId,
) {
    if let Some(buff) = ctx.uad.src.get_a_buff(a_buff_id) {
        for buff_mod in buff.mods.iter() {
            let modifier = match RawModifier::try_from_a_buff_regular(
                item_key,
                item,
                a_effect,
                buff,
                buff_mod,
                buff_val_a_attr_id,
                a_buff_scope.into(),
                buff_type_a_attr_id,
            ) {
                Some(modifier) => modifier,
                None => continue,
            };
            modifiers.push(modifier);
        }
    }
}

fn add_buff_mods_hardcoded(
    modifiers: &mut Vec<RawModifier>,
    ctx: &SvcCtx,
    item_key: ItemKey,
    item: &UadItem,
    a_effect: &ad::AEffectRt,
    a_buff_id: &ad::ABuffId,
    a_buff_scope: &ad::AEffectBuffScope,
    buff_a_val: ad::AAttrVal,
) {
    if let Some(buff) = ctx.uad.src.get_a_buff(a_buff_id) {
        for buff_mod in buff.mods.iter() {
            let modifier = match RawModifier::try_from_a_buff_hardcoded(
                item_key,
                item,
                a_effect,
                buff,
                buff_mod,
                buff_a_val,
                a_buff_scope.into(),
            ) {
                Some(modifier) => modifier,
                None => continue,
            };
            modifiers.push(modifier);
        }
    }
}
