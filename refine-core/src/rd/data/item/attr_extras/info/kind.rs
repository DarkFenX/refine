use crate::{
    ad::{AItemCatId, AItemGrpId},
    misc::DetectedItemKind,
    num::Value,
    rd::{RAttrConsts, RAttrId, REffectConsts, REffectId, RItemEffectData},
    util::RMap,
};

pub(in crate::rd::data::item::attr_extras) fn detect_item_kind(
    item_grp_id: AItemGrpId,
    item_cat_id: AItemCatId,
    item_attrs: &RMap<RAttrId, Value>,
    item_effects: &RMap<REffectId, RItemEffectData>,
    attr_consts: &RAttrConsts,
    effect_consts: &REffectConsts,
) -> Option<DetectedItemKind> {
    let mut store = ItemKindStore::new();
    match item_cat_id {
        // Ship & structure modules
        AItemCatId::MODULE | AItemCatId::STRUCTURE_MODULE => {
            if let Some(effect_rid) = effect_consts.hi_power
                && item_effects.contains_key(&effect_rid)
            {
                store.push(DetectedItemKind::ModuleHigh)?;
            }
            if let Some(effect_rid) = effect_consts.med_power
                && item_effects.contains_key(&effect_rid)
            {
                store.push(DetectedItemKind::ModuleMid)?;
            }
            if let Some(effect_rid) = effect_consts.lo_power
                && item_effects.contains_key(&effect_rid)
            {
                store.push(DetectedItemKind::ModuleLow)?;
            }
            if let Some(effect_rid) = effect_consts.rig_slot
                && item_effects.contains_key(&effect_rid)
            {
                store.push(DetectedItemKind::Rig)?;
            }
            if let Some(effect_rid) = effect_consts.service_slot
                && item_effects.contains_key(&effect_rid)
            {
                store.push(DetectedItemKind::Service)?;
            }
        }
        // Ships and structures
        AItemCatId::SHIP | AItemCatId::STRUCTURE => store.push(DetectedItemKind::Ship)?,
        // Implants and boosters
        AItemCatId::IMPLANT => {
            if let Some(attr_rid) = attr_consts.boosterness
                && item_attrs.contains_key(&attr_rid)
            {
                store.push(DetectedItemKind::Booster)?;
            }
            if let Some(attr_rid) = attr_consts.implantness
                && item_attrs.contains_key(&attr_rid)
            {
                store.push(DetectedItemKind::Implant)?;
            }
        }
        // Other items
        AItemCatId::CHARGE => store.push(DetectedItemKind::Charge)?,
        AItemCatId::DRONE => store.push(DetectedItemKind::Drone)?,
        AItemCatId::FIGHTER => store.push(DetectedItemKind::Fighter)?,
        AItemCatId::SKILL => store.push(DetectedItemKind::Skill)?,
        AItemCatId::SUBSYSTEM => store.push(DetectedItemKind::Subsystem)?,
        _ => (),
    }
    match item_grp_id {
        AItemGrpId::CHARACTER => store.push(DetectedItemKind::Character)?,
        AItemGrpId::SHIP_MODIFIER => store.push(DetectedItemKind::Stance)?,
        _ => (),
    }
    store.extract()
}

struct ItemKindStore {
    data: Option<DetectedItemKind>,
}
impl ItemKindStore {
    fn new() -> Self {
        Self { data: None }
    }
    fn push(&mut self, item_kind: DetectedItemKind) -> Option<()> {
        match self.data {
            Some(_) => None,
            None => {
                self.data = Some(item_kind);
                Some(())
            }
        }
    }
    fn extract(self) -> Option<DetectedItemKind> {
        self.data
    }
}
