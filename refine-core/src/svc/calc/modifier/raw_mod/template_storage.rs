use super::{
    affector::{RawModAffector, RawModItemDependent},
    id::{RawModId, RawModTemplateId},
    raw_mod::RawModifier,
};
use crate::{
    rd::{RAttrConsts, RAttrId, RBuff, RBuffId, REffect, REffectId},
    svc::calc::ModifierKind,
    ud::UItemId,
    util::{DenseId, PDense},
};

/// Modifiers which do not depend on the item running their effect.
///
/// Built once per runtime data. They are not stored per item: template and affector item in the
/// modifier ID are enough to get the modifier.
pub(crate) struct RawModTemplates(Box<[RawModifier]>);
impl RawModTemplates {
    /// Fill templates, removing modifiers from their source, since they won't be needed there
    /// anymore.
    pub(crate) fn from_effects(
        effects: &mut PDense<REffectId, REffect>,
        buffs: &PDense<RBuffId, RBuff>,
        attr_consts: &RAttrConsts,
    ) -> Self {
        let mut templates = Vec::new();
        for effect in effects.values_mut() {
            let start = templates.len();
            fill_effect_templates(&mut templates, effect, attr_consts);
            fill_buff_templates(&mut templates, effect, buffs);
            effect.rmod_templates = start as u32..templates.len() as u32;
            // Mark effects which still have some modifiers left for generation
            effect.needs_rmod_gen = !effect.modifiers.is_empty()
                || effect.calc_custom_mod.is_some()
                || effect
                    .buff
                    .as_ref()
                    .is_some_and(|buff| buff.attr_merge.is_some() || !buff.full.is_empty());
        }
        Self(templates.into())
    }
    pub(in crate::svc::calc) fn get(&self, template_id: RawModTemplateId, item_uid: UItemId) -> RawModifier {
        self.0[template_id.index()].with_affector_uid(item_uid)
    }
    pub(in crate::svc::calc) fn iter_effect(
        &self,
        effect: &REffect,
        item_uid: UItemId,
    ) -> impl ExactSizeIterator<Item = (RawModId, RawModifier)> {
        let start = effect.rmod_templates.start as usize;
        let end = effect.rmod_templates.end as usize;
        self.0[start..end].iter().enumerate().map(move |(i, template)| {
            (
                RawModId::from_template(RawModTemplateId::new(start + i), item_uid),
                template.with_affector_uid(item_uid),
            )
        })
    }
}

// Modifiers which depend on the item running the effect stay on the effect
fn fill_effect_templates(templates: &mut Vec<RawModifier>, effect: &mut REffect, attr_consts: &RAttrConsts) {
    let mut effect_mods = std::mem::take(&mut effect.modifiers).into_vec();
    effect_mods.retain(|effect_mod| {
        match RawModifier::try_from_effect_mod(RawModAffector::Template, effect, effect_mod) {
            Ok(Some(template)) => {
                templates.push(template);
                false
            }
            Ok(None) => false,
            Err(RawModItemDependent) => true,
        }
    });
    effect.modifiers = effect_mods.into_boxed_slice();
    if let Some(custom_mod) = effect.calc_custom_mod {
        match custom_mod.make_rmod(attr_consts, RawModAffector::Template, effect) {
            Ok(Some(template)) => {
                templates.push(template);
                effect.calc_custom_mod = None;
            }
            Ok(None) => effect.calc_custom_mod = None,
            Err(RawModItemDependent) => (),
        }
    }
}

// Buff definitions with a modifier which depends on the item running the effect stay on the effect
fn fill_buff_templates(templates: &mut Vec<RawModifier>, effect: &mut REffect, buffs: &PDense<RBuffId, RBuff>) {
    let Some(effect_buff) = &mut effect.buff else {
        return;
    };
    let mut buff_fulls = std::mem::take(&mut effect_buff.full).into_vec();
    buff_fulls.retain(|buff_full| {
        // Buff modifiers usually fail all together (e.g. on item-specific resistance check), so do
        // not handle them one-by-one, stick to per-buff granularity
        let start = templates.len();
        let buff = buffs.get(buff_full.buff_rid).unwrap();
        for buff_mod in buff.mods.iter() {
            match RawModifier::try_from_buff_full_mod(RawModAffector::Template, effect, buff, buff_full, buff_mod) {
                Ok(Some(template)) => templates.push(template),
                Ok(None) => (),
                Err(RawModItemDependent) => {
                    templates.truncate(start);
                    return true;
                }
            }
        }
        false
    });
    effect.buff.as_mut().unwrap().full = buff_fulls.into();
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Related to local-modifier-affector optimization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RawModTemplates {
    pub(crate) fn iter_local_affector_attr_rids(&self) -> impl Iterator<Item = RAttrId> {
        self.0
            .iter()
            .filter(|template| matches!(template.kind, ModifierKind::Local))
            .filter_map(|template| template.get_affector_attr_rid())
    }
    pub(crate) fn iter_effect_local_affector_attr_rids(&self, effect: &REffect) -> impl Iterator<Item = RAttrId> {
        let start = effect.rmod_templates.start as usize;
        let end = effect.rmod_templates.end as usize;
        self.0[start..end]
            .iter()
            .filter(|template| matches!(template.kind, ModifierKind::Local))
            .filter_map(|template| template.get_affector_attr_rid())
    }
    pub(in crate::svc::calc) fn iter_effect_local_by_affector_attr(
        &self,
        effect: &REffect,
        item_uid: UItemId,
        affector_attr_rid: RAttrId,
    ) -> impl Iterator<Item = RawModifier> {
        let start = effect.rmod_templates.start as usize;
        let end = effect.rmod_templates.end as usize;
        self.0[start..end]
            .iter()
            .filter(move |template| {
                matches!(template.kind, ModifierKind::Local)
                    && template.get_affector_attr_rid() == Some(affector_attr_rid)
            })
            .map(move |template| template.with_affector_uid(item_uid))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;

    impl RawModTemplates {
        pub(in crate::svc::calc) fn try_get(
            &self,
            template_id: RawModTemplateId,
            item_uid: UItemId,
        ) -> Option<RawModifier> {
            self.0
                .get(template_id.index())
                .map(|template| template.with_affector_uid(item_uid))
        }
    }
}
