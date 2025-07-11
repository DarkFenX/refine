use crate::{
    ac, ad,
    def::ItemKey,
    misc::EffectSpec,
    svc::{
        SvcCtx,
        calc::{Calc, RawModifier},
    },
    uad::UadItem,
};

impl Calc {
    pub(super) fn generate_mods_for_effect(
        &mut self,
        reuse_rmods: &mut Vec<RawModifier>,
        ctx: SvcCtx,
        item_key: ItemKey,
        item: &UadItem,
        a_effect: &ad::AEffectRt,
    ) {
        reuse_rmods.clear();
        // Regular modifiers
        for a_mod in a_effect.ae.mods.iter() {
            match RawModifier::try_from_amod(item_key, item, a_effect, a_mod) {
                Some(sol_mod) => reuse_rmods.push(sol_mod),
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
                                reuse_rmods,
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
                                reuse_rmods,
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
                                reuse_rmods,
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
        if let Some(customizer) = a_effect.hc.calc_custom_fn {
            customizer(reuse_rmods, EffectSpec::new(item_key, a_effect.ae.id));
        }
    }
    pub(super) fn generate_dependent_buff_mods<'a>(
        &mut self,
        ctx: SvcCtx,
        item_key: ItemKey,
        item: &UadItem,
        a_effect_ids: impl Iterator<Item = &'a ad::AEffectId>,
        buff_type_a_attr_id: ad::AAttrId,
    ) -> Vec<RawModifier> {
        let mut rmods = Vec::new();
        let buff_value_a_attr_id = match buff_type_a_attr_id {
            ac::attrs::WARFARE_BUFF1_ID => ac::attrs::WARFARE_BUFF1_VAL,
            ac::attrs::WARFARE_BUFF2_ID => ac::attrs::WARFARE_BUFF2_VAL,
            ac::attrs::WARFARE_BUFF3_ID => ac::attrs::WARFARE_BUFF3_VAL,
            ac::attrs::WARFARE_BUFF4_ID => ac::attrs::WARFARE_BUFF4_VAL,
            _ => return rmods,
        };
        for a_effect_id in a_effect_ids {
            let a_effect = ctx.uad.src.get_a_effect(a_effect_id).unwrap();
            if let Some(a_buff_info) = a_effect.ae.buff.as_ref()
                && matches!(a_buff_info.source, ad::AEffectBuffSrc::DefaultAttrs)
                && let Ok(buff_id_cval) = self.get_item_attr_val_full(ctx, item_key, &buff_type_a_attr_id)
            {
                add_buff_mods(
                    &mut rmods,
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
        rmods
    }
}

fn add_buff_mods(
    rmods: &mut Vec<RawModifier>,
    ctx: SvcCtx,
    item_key: ItemKey,
    item: &UadItem,
    a_effect: &ad::AEffectRt,
    a_buff_id: &ad::ABuffId,
    a_buff_scope: &ad::AEffectBuffScope,
    buff_type_a_attr_id: Option<ad::AAttrId>,
    buff_val_a_attr_id: ad::AAttrId,
) {
    let a_buff = match ctx.uad.src.get_a_buff(a_buff_id) {
        Some(a_buff) => a_buff,
        None => return,
    };
    for buff_mod in a_buff.mods.iter() {
        let rmod = match RawModifier::try_from_a_buff_regular(
            item_key,
            item,
            a_effect,
            a_buff,
            buff_mod,
            buff_val_a_attr_id,
            a_buff_scope.into(),
            buff_type_a_attr_id,
        ) {
            Some(rmod) => rmod,
            None => continue,
        };
        rmods.push(rmod);
    }
}

fn add_buff_mods_hardcoded(
    rmods: &mut Vec<RawModifier>,
    ctx: SvcCtx,
    item_key: ItemKey,
    item: &UadItem,
    a_effect: &ad::AEffectRt,
    a_buff_id: &ad::ABuffId,
    a_buff_scope: &ad::AEffectBuffScope,
    buff_a_val: ad::AAttrVal,
) {
    let a_buff = match ctx.uad.src.get_a_buff(a_buff_id) {
        Some(a_buff) => a_buff,
        None => return,
    };
    for buff_mod in a_buff.mods.iter() {
        let rmod = match RawModifier::try_from_a_buff_hardcoded(
            item_key,
            item,
            a_effect,
            a_buff,
            buff_mod,
            buff_a_val,
            a_buff_scope.into(),
        ) {
            Some(rmod) => rmod,
            None => continue,
        };
        rmods.push(rmod);
    }
}
