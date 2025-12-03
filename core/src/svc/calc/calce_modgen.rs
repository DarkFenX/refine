use crate::{
    ac,
    ad::{AAttrId, AAttrVal, ABuffId, AEffectBuffScope, AEffectBuffStrength},
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
            // Buffs which are partially defined and rely on on-item attributes to complete
            // definition
            if let Some(buff_attr_merge) = buff_info.attr_merge {
                for (buff_type_attr_id, buff_val_attr_id) in ac::extras::BUFF_MERGE_ATTRS {
                    if let Ok(buff_id) = self.get_item_attr_val_full(ctx, item_key, &buff_type_attr_id) {
                        add_buff_mods_with_attr(
                            reuse_rmods,
                            ctx,
                            item_key,
                            item,
                            effect,
                            &(buff_id.extra.round() as ABuffId),
                            &buff_attr_merge.scope,
                            Some(buff_type_attr_id),
                            buff_val_attr_id,
                        );
                    }
                }
            }
            // Fully defined buffs
            for buff_full in buff_info.full.iter() {
                match buff_full.strength {
                    AEffectBuffStrength::Attr(buff_val_attr_id) => add_buff_mods_with_attr(
                        reuse_rmods,
                        ctx,
                        item_key,
                        item,
                        effect,
                        &buff_full.buff_id,
                        &buff_full.scope,
                        None,
                        buff_val_attr_id,
                    ),
                    AEffectBuffStrength::Hardcoded(buff_val) => add_buff_mods_with_hardcoded(
                        reuse_rmods,
                        ctx,
                        item_key,
                        item,
                        effect,
                        &buff_full.buff_id,
                        &buff_full.scope,
                        buff_val,
                    ),
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
                && let Some(buff_attr_merge) = buff_info.attr_merge
                && let Ok(buff_id_cval) = self.get_item_attr_val_full(ctx, item_key, &buff_type_attr_id)
            {
                add_buff_mods_with_attr(
                    &mut rmods,
                    ctx,
                    item_key,
                    item,
                    effect,
                    &(buff_id_cval.extra.round() as ABuffId),
                    &buff_attr_merge.scope,
                    Some(buff_type_attr_id),
                    buff_value_attr_id,
                );
            }
        }
        rmods
    }
}

fn add_buff_mods_with_attr(
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
        let rmod = match RawModifier::try_from_buff_with_attr(
            item_key,
            item,
            effect,
            buff,
            buff_scope,
            buff_mod,
            buff_val_attr_id,
            buff_type_attr_id,
        ) {
            Some(rmod) => rmod,
            None => continue,
        };
        rmods.push(rmod);
    }
}

fn add_buff_mods_with_hardcoded(
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
        let rmod = match RawModifier::try_from_buff_with_hardcoded(
            item_key, item, effect, buff, buff_scope, buff_mod, buff_val,
        ) {
            Some(rmod) => rmod,
            None => continue,
        };
        rmods.push(rmod);
    }
}
