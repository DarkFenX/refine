use crate::{
    ad::{AAttrVal, ABuffId},
    misc::EffectSpec,
    rd::{RAttrKey, RBuff, REffect, REffectBuffScope, REffectBuffStrength, REffectKey},
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
        for effect_mod in effect.mods.iter() {
            match RawModifier::try_from_effect_mod(item_key, item, effect, effect_mod) {
                Some(raw_mod) => reuse_rmods.push(raw_mod),
                None => continue,
            };
        }
        // Buffs
        if let Some(buff_info) = &effect.buff_info {
            // Buffs which are partially defined and rely on on-item attributes to complete
            // definition
            if let Some(buff_attr_merge) = &buff_info.attr_merge {
                for (buff_type_attr_key, buff_str_attr_key) in ctx.ac().buff_merge_ids_strs.iter() {
                    if let Ok(buff_id) = self.get_item_attr_rfull(ctx, item_key, *buff_type_attr_key) {
                        let buff_id = buff_id.extra.round() as ABuffId;
                        let buff = match ctx.u_data.src.get_buff_by_id(&buff_id) {
                            Some(buff) => buff,
                            None => continue,
                        };
                        add_buff_mods_with_attr(
                            reuse_rmods,
                            item_key,
                            item,
                            effect,
                            buff,
                            &buff_attr_merge.scope,
                            Some(*buff_type_attr_key),
                            *buff_str_attr_key,
                        );
                    }
                }
            }
            // Fully defined buffs
            for buff_full in buff_info.full.iter() {
                match buff_full.strength {
                    REffectBuffStrength::Attr(buff_str_attr_key) => {
                        let buff = ctx.u_data.src.get_buff(buff_full.buff_key);
                        add_buff_mods_with_attr(
                            reuse_rmods,
                            item_key,
                            item,
                            effect,
                            buff,
                            &buff_full.scope,
                            None,
                            buff_str_attr_key,
                        )
                    }
                    REffectBuffStrength::Hardcoded(buff_str) => {
                        let buff = ctx.u_data.src.get_buff(buff_full.buff_key);
                        add_buff_mods_with_hardcoded(
                            reuse_rmods,
                            item_key,
                            item,
                            effect,
                            buff,
                            &buff_full.scope,
                            buff_str,
                        )
                    }
                }
            }
        }
        // Custom modifiers
        if let Some(customizer) = effect.calc_customizer {
            customizer(
                reuse_rmods,
                &ctx.u_data.src.get_attr_consts(),
                EffectSpec::new(item_key, effect.key),
            );
        }
    }
    pub(super) fn generate_dependent_buff_mods<'a>(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        item: &UItem,
        effect_keys: impl Iterator<Item = &'a REffectKey>,
        buff_type_attr_key: RAttrKey,
    ) -> Vec<RawModifier> {
        let mut rmods = Vec::new();
        let buff_str_attr_key = match ctx.u_data.src.get_attr(buff_type_attr_key).buff_str_attr_key {
            Some(buff_str_attr_key) => buff_str_attr_key,
            _ => return rmods,
        };
        for &effect_key in effect_keys {
            let effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(buff_info) = &effect.buff_info
                && let Some(buff_attr_merge) = &buff_info.attr_merge
                && let Ok(buff_id_cval) = self.get_item_attr_rfull(ctx, item_key, buff_type_attr_key)
            {
                let buff_id = buff_id_cval.extra.round() as ABuffId;
                let buff = match ctx.u_data.src.get_buff_by_id(&buff_id) {
                    Some(buff) => buff,
                    None => continue,
                };
                add_buff_mods_with_attr(
                    &mut rmods,
                    item_key,
                    item,
                    effect,
                    buff,
                    &buff_attr_merge.scope,
                    Some(buff_type_attr_key),
                    buff_str_attr_key,
                );
            }
        }
        rmods
    }
}

fn add_buff_mods_with_attr(
    rmods: &mut Vec<RawModifier>,
    item_key: UItemKey,
    item: &UItem,
    effect: &REffect,
    buff: &RBuff,
    buff_scope: &REffectBuffScope,
    buff_type_attr_key: Option<RAttrKey>,
    buff_str_attr_key: RAttrKey,
) {
    for buff_mod in buff.mods.iter() {
        let rmod = match RawModifier::try_from_buff_with_attr(
            item_key,
            item,
            effect,
            buff,
            buff_scope,
            buff_mod,
            buff_type_attr_key,
            buff_str_attr_key,
        ) {
            Some(rmod) => rmod,
            None => continue,
        };
        rmods.push(rmod);
    }
}

fn add_buff_mods_with_hardcoded(
    rmods: &mut Vec<RawModifier>,
    item_key: UItemKey,
    item: &UItem,
    effect: &REffect,
    buff: &RBuff,
    buff_scope: &REffectBuffScope,
    buff_str: AAttrVal,
) {
    for buff_mod in buff.mods.iter() {
        let rmod = match RawModifier::try_from_buff_with_hardcoded(
            item_key, item, effect, buff, buff_scope, buff_mod, buff_str,
        ) {
            Some(rmod) => rmod,
            None => continue,
        };
        rmods.push(rmod);
    }
}
