use crate::{
    ad,
    sol::{
        ItemId,
        svc::{AttrSpec, EffectSpec},
    },
    util::StMapSetL1,
};

// Intended to hold ad-hoc dependencies between attributes, which are not covered by registers
// which hold data about regular modifiers.
#[derive(Clone)]
pub(in crate::sol::svc::calc) struct DependencyRegister {
    // Map<affector spec, affectee specs> - this map could be StMapSetL2 with Option<EffectSpec> as
    // source in 3rd generic parameter, just to process collisions - i.e. when the same affector
    // attribute and affectee attribute are used for anonymous and source-based dependency. But in
    // our case it's not needed with some external guarantees:
    // - when a modification source is removed, it clears dependent attribute values
    // - anonymous dependencies are re-established on every dependent recalculation
    // So, if effect source is removed (e.g. disabled via force stop mode), it clears up affectee
    // attribute, and when requested next time - it will re-add anonymous dependency, allowing the
    // affectee attribute to be cleared whenever linked attribute changes its value.
    pub(super) data: StMapSetL1<AttrSpec, AttrSpec>,
    // Map<item ID, (affector attr ID, affectee attr ID)>
    pub(super) anonymous_by_item: StMapSetL1<ItemId, (ad::AAttrId, ad::AAttrId)>,
    // Map<source, (affector spec, affectee spec)>
    pub(super) by_source: StMapSetL1<EffectSpec, (AttrSpec, AttrSpec)>,
    // Map<item ID, sources>
    pub(super) source_by_item: StMapSetL1<ItemId, EffectSpec>,
}
impl DependencyRegister {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self {
            data: StMapSetL1::new(),
            anonymous_by_item: StMapSetL1::new(),
            by_source: StMapSetL1::new(),
            source_by_item: StMapSetL1::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::calc) fn get_affectee_attr_specs(
        &self,
        affector_item_id: &ItemId,
        affector_a_attr_id: &ad::AAttrId,
    ) -> impl ExactSizeIterator<Item = &AttrSpec> {
        let affector_spec = AttrSpec {
            item_id: *affector_item_id,
            a_attr_id: *affector_a_attr_id,
        };
        self.data.get(&affector_spec)
    }
    // Modification methods
    pub(in crate::sol::svc::calc) fn add_anonymous(
        &mut self,
        item_id: ItemId,
        affector_a_attr_id: ad::AAttrId,
        affectee_a_attr_id: ad::AAttrId,
    ) {
        let affector_spec = AttrSpec {
            item_id,
            a_attr_id: affector_a_attr_id,
        };
        let affectee_spec = AttrSpec {
            item_id,
            a_attr_id: affectee_a_attr_id,
        };
        self.data.add_entry(affector_spec, affectee_spec);
        self.anonymous_by_item
            .add_entry(item_id, (affector_a_attr_id, affectee_a_attr_id));
    }
    pub(in crate::sol::svc::calc) fn add_with_source(
        &mut self,
        source_item_id: ItemId,
        source_a_effect_id: ad::AEffectId,
        affector_item_id: ItemId,
        affector_a_attr_id: ad::AAttrId,
        affectee_item_id: ItemId,
        affectee_a_attr_id: ad::AAttrId,
    ) {
        let source = EffectSpec {
            item_id: source_item_id,
            a_effect_id: source_a_effect_id,
        };
        let affector_spec = AttrSpec {
            item_id: affector_item_id,
            a_attr_id: affector_a_attr_id,
        };
        let affectee_spec = AttrSpec {
            item_id: affectee_item_id,
            a_attr_id: affectee_a_attr_id,
        };
        self.data.add_entry(affector_spec, affectee_spec);
        self.by_source.add_entry(source, (affector_spec, affectee_spec));
        self.source_by_item.add_entry(affector_item_id, source);
        self.source_by_item.add_entry(affectee_item_id, source);
    }
    pub(in crate::sol::svc::calc) fn remove_by_source(
        &mut self,
        source_item_id: ItemId,
        source_a_effect_id: ad::AEffectId,
    ) {
        let source = EffectSpec {
            item_id: source_item_id,
            a_effect_id: source_a_effect_id,
        };
        if let Some(spec_iter) = self.by_source.remove_key(&source) {
            for (affector_spec, affectee_spec) in spec_iter {
                self.data.remove_entry(&affector_spec, &affectee_spec);
                self.source_by_item.remove_entry(&affector_spec.item_id, &source);
                self.source_by_item.remove_entry(&affectee_spec.item_id, &source);
            }
        }
    }
    pub(in crate::sol::svc::calc) fn remove_item(&mut self, item_id: &ItemId) {
        // Anonymous dependencies
        if let Some(attrs_iter) = self.anonymous_by_item.remove_key(item_id) {
            for (affector_a_attr_id, affectee_a_attr_id) in attrs_iter {
                let affector_spec = AttrSpec {
                    item_id: *item_id,
                    a_attr_id: affector_a_attr_id,
                };
                let affectee_spec = AttrSpec {
                    item_id: *item_id,
                    a_attr_id: affectee_a_attr_id,
                };
                self.data.remove_entry(&affector_spec, &affectee_spec);
            }
        }
        // Dependencies with source
        if let Some(sources) = self.source_by_item.remove_key(item_id) {
            for source in sources {
                if let Some(attr_spec_iter) = self.by_source.remove_key(&source) {
                    for (affector_spec, affectee_spec) in attr_spec_iter {
                        self.data.remove_entry(&affector_spec, &affectee_spec);
                    }
                }
            }
        }
    }
}
