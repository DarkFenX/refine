use super::{
    id::{RawModId, RawModTemplateId},
    raw_mod::RawModifier,
    template_status::RawModTemplateStatus,
};
use crate::{
    rd::{RAttrConsts, RBuff, RBuffId, REffect, REffectId},
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
    effect_mods.retain(
        |effect_mod| match RawModifier::make_template_from_effect_mod(effect, effect_mod) {
            RawModTemplateStatus::Template(template) => {
                templates.push(template);
                false
            }
            RawModTemplateStatus::NoModifier => false,
            RawModTemplateStatus::ItemDependent => true,
        },
    );
    effect.modifiers = effect_mods.into_boxed_slice();
    // All of custom modifiers can be templated so far (have no self-skillreq affectee filters, have
    // no projection resistances which depend on source item), so add them as well
    if let Some(custom_mod) = effect.calc_custom_mod
        && let Some(template) = custom_mod.make_rmod(attr_consts, RawModifier::make_template_espec(effect))
    {
        templates.push(template);
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
            match RawModifier::make_template_from_buff_mod(effect, buff, buff_full, buff_mod) {
                RawModTemplateStatus::Template(template) => templates.push(template),
                RawModTemplateStatus::NoModifier => (),
                RawModTemplateStatus::ItemDependent => {
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
