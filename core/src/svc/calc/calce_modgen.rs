use crate::{
    ac,
    ad::{AAttrId, AAttrVal, ABuffId, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom},
    misc::EffectSpec,
    rd::{REffect, REffectKey},
    svc::{
        SvcCtx,
        calc::{Calc, RawModifier},
    },
    ud::{UItem, UItemKey},
};

impl Calc {
    pub(super) fn generate_mods_for_effect(
        &mut self,
        reuse_rmods: &mut Vec<RawModifier>,
        ctx: SvcCtx,
        item_key: UItemKey,
        item: &UItem,
        effect: &REffect,
    ) {
        reuse_rmods.clear();
        // Regular modifiers
        for a_mod in effect.get_mods().iter() {
            match RawModifier::try_from_effect_mod(item_key, item, effect, a_mod) {
                Some(raw_mod) => reuse_rmods.push(raw_mod),
                None => continue,
            };
        }
        // Buffs
        if let Some(buff_info) = effect.get_buff_info().as_ref() {
            match &buff_info.source {
                AEffectBuffSrc::DefaultAttrs => {
                    for (buff_type_attr_id, buff_val_attr_id) in ac::extras::BUFF_STDATTRS {
                        if let Ok(buff_id) = self.get_item_attr_val_full(ctx, item_key, &buff_type_attr_id) {
                            add_buff_mods(
                                reuse_rmods,
                                ctx,
                                item_key,
                                item,
                                effect,
                                &(buff_id.extra.round() as ABuffId),
                                &buff_info.scope,
                                Some(buff_type_attr_id),
                                buff_val_attr_id,
                            );
                        }
                    }
                }
                AEffectBuffSrc::Customized(buff_custom_srcs) => {
                    for buff_custom_src in buff_custom_srcs {
                        match buff_custom_src {
                            AEffectBuffSrcCustom::AffectorVal(buff_id, buff_val_attr_id) => add_buff_mods(
                                reuse_rmods,
                                ctx,
                                item_key,
                                item,
                                effect,
                                buff_id,
                                &buff_info.scope,
                                None,
                                *buff_val_attr_id,
                            ),
                            AEffectBuffSrcCustom::HardcodedVal(buff_id, buff_val) => add_buff_mods_hardcoded(
                                reuse_rmods,
                                ctx,
                                item_key,
                                item,
                                effect,
                                buff_id,
                                &buff_info.scope,
                                *buff_val,
                            ),
                        }
                    }
                }
            }
        }
        // Custom modifiers
        if let Some(customizer) = effect.get_calc_customizer() {
            customizer(reuse_rmods, EffectSpec::new(item_key, effect.get_key()));
        }
    }
    pub(super) fn generate_dependent_buff_mods<'a>(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        item: &UItem,
        effect_keys: impl Iterator<Item = &'a REffectKey>,
        buff_type_attr_id: AAttrId,
    ) -> Vec<RawModifier> {
        let mut rmods = Vec::new();
        let buff_value_attr_id = match buff_type_attr_id {
            ac::attrs::WARFARE_BUFF1_ID => ac::attrs::WARFARE_BUFF1_VAL,
            ac::attrs::WARFARE_BUFF2_ID => ac::attrs::WARFARE_BUFF2_VAL,
            ac::attrs::WARFARE_BUFF3_ID => ac::attrs::WARFARE_BUFF3_VAL,
            ac::attrs::WARFARE_BUFF4_ID => ac::attrs::WARFARE_BUFF4_VAL,
            _ => return rmods,
        };
        for &effect_key in effect_keys {
            let effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(buff_info) = effect.get_buff_info().as_ref()
                && matches!(buff_info.source, AEffectBuffSrc::DefaultAttrs)
                && let Ok(buff_id_cval) = self.get_item_attr_val_full(ctx, item_key, &buff_type_attr_id)
            {
                add_buff_mods(
                    &mut rmods,
                    ctx,
                    item_key,
                    item,
                    effect,
                    &(buff_id_cval.extra.round() as ABuffId),
                    &buff_info.scope,
                    Some(buff_type_attr_id),
                    buff_value_attr_id,
                );
            }
        }
        rmods
    }
}

fn add_buff_mods(
    rmods: &mut Vec<RawModifier>,
    ctx: SvcCtx,
    item_key: UItemKey,
    item: &UItem,
    effect: &REffect,
    buff_id: &ABuffId,
    buff_scope: &AEffectBuffScope,
    buff_type_attr_id: Option<AAttrId>,
    buff_val_attr_id: AAttrId,
) {
    let buff = match ctx.u_data.src.get_buff(buff_id) {
        Some(buff) => buff,
        None => return,
    };
    for buff_mod in buff.get_mods().iter() {
        let rmod = match RawModifier::try_from_buff_regular(
            item_key,
            item,
            effect,
            buff,
            buff_mod,
            buff_val_attr_id,
            buff_scope.into(),
            buff_type_attr_id,
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
    item_key: UItemKey,
    item: &UItem,
    effect: &REffect,
    buff_id: &ABuffId,
    buff_scope: &AEffectBuffScope,
    buff_val: AAttrVal,
) {
    let buff = match ctx.u_data.src.get_buff(buff_id) {
        Some(buff) => buff,
        None => return,
    };
    for buff_mod in buff.get_mods().iter() {
        let rmod = match RawModifier::try_from_buff_hardcoded(
            item_key,
            item,
            effect,
            buff,
            buff_mod,
            buff_val,
            buff_scope.into(),
        ) {
            Some(rmod) => rmod,
            None => continue,
        };
        rmods.push(rmod);
    }
}
