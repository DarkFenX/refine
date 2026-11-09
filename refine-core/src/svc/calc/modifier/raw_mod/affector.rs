use crate::{
    ad::{AItemId, AModifierSrq},
    misc::EffectSpec,
    rd::{RAttrId, REffect, REffectResist},
    ud::{UData, UItem, UItemId},
    util::DenseId,
};

/// Item running the effect, and the only way raw modifier construction can read it.
///
/// Templates are built without one, so whatever needs it makes the modifier item-dependent.
#[derive(Copy, Clone)]
pub(in crate::svc::calc) enum RawModAffector<'a> {
    Template,
    Item {
        u_data: &'a UData,
        item_uid: UItemId,
        item: &'a UItem,
    },
}
impl<'a> RawModAffector<'a> {
    pub(in crate::svc::calc) fn new(u_data: &'a UData, item_uid: UItemId, item: &'a UItem) -> Self {
        Self::Item { u_data, item_uid, item }
    }
    pub(in crate::svc::calc::modifier) fn get_espec(self, effect: &REffect) -> EffectSpec {
        match self {
            // Item is replaced when a template gets resolved, so the value does not matter
            Self::Template => EffectSpec::new(UItemId::new(0), effect.rid),
            Self::Item { item_uid, .. } => EffectSpec::new(item_uid, effect.rid),
        }
    }
    pub(in crate::svc::calc::modifier) fn get_srq_type_aid(
        self,
        mod_srq: &AModifierSrq,
    ) -> Result<AItemId, RawModItemDependent> {
        match (mod_srq, self) {
            (AModifierSrq::ItemId(type_aid), ..) => Ok(*type_aid),
            (AModifierSrq::SelfRef, Self::Template) => Err(RawModItemDependent),
            (AModifierSrq::SelfRef, Self::Item { item, .. }) => Ok(item.get_rendered_type_aid()),
        }
    }
    pub(in crate::svc::calc::modifier) fn get_resist_attr_rid(
        self,
        resist: REffectResist,
    ) -> Result<Option<RAttrId>, RawModItemDependent> {
        match (resist, self) {
            (REffectResist::Attr(attr_rid), ..) => Ok(Some(attr_rid)),
            (REffectResist::AttrRef(..) | REffectResist::RemoteResistance, Self::Template) => Err(RawModItemDependent),
            (.., Self::Item { u_data, item_uid, .. }) => Ok(resist.get_attr_rid(u_data, item_uid)),
        }
    }
}

/// This "error" means raw modifier construction needs the item running the effect, and thus is
/// invalid to be a template.
pub(in crate::svc::calc) struct RawModItemDependent;
