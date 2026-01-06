use crate::{
    misc::{AttrSpec, EffectSpec},
    rd::RAttrId,
    ud::UItemId,
    util::RMapRSet,
};

// Intended to hold ad-hoc dependencies between attributes, which are not covered by registers
// which hold data about regular modifiers.
#[derive(Clone)]
pub(crate) struct DependencyRegister {
    // Map<affector spec, affectee specs> - this map could be StMapSetL2 with Option<EffectSpec> as
    // source in 3rd generic parameter, just to process collisions - i.e. when the same affector
    // attribute and affectee attribute are used for anonymous and source-based dependency. But in
    // our case it's not needed with some external guarantees:
    // - when a modification source is removed, it clears dependent attribute values
    // - anonymous dependencies are re-established on every dependent recalculation
    // So, if effect source is removed (e.g. disabled via force stop mode), it clears up affectee
    // attribute, and when requested next time - it will re-add anonymous dependency, allowing the
    // affectee attribute to be cleared whenever linked attribute changes its value.
    pub(super) data: RMapRSet<AttrSpec, AttrSpec>,
    // Map<item ID, (affector attr RID, affectee attr RID)>
    pub(super) anonymous_by_item: RMapRSet<UItemId, (RAttrId, RAttrId)>,
    // Map<source, (affector spec, affectee spec)>
    pub(super) by_source: RMapRSet<EffectSpec, (AttrSpec, AttrSpec)>,
    // Map<item ID, sources>
    pub(super) source_by_item: RMapRSet<UItemId, EffectSpec>,
}
impl DependencyRegister {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            data: RMapRSet::new(),
            anonymous_by_item: RMapRSet::new(),
            by_source: RMapRSet::new(),
            source_by_item: RMapRSet::new(),
        }
    }
    // Query methods
    pub(in crate::svc::calc) fn iter_affectee_aspecs(
        &self,
        affector_aspec: &AttrSpec,
    ) -> impl ExactSizeIterator<Item = &AttrSpec> {
        self.data.get(affector_aspec)
    }
    // Modification methods
    pub(in crate::svc::calc) fn add_anonymous(
        &mut self,
        item_uid: UItemId,
        affector_attr_rid: RAttrId,
        affectee_attr_rid: RAttrId,
    ) {
        let affector_spec = AttrSpec::new(item_uid, affector_attr_rid);
        let affectee_spec = AttrSpec::new(item_uid, affectee_attr_rid);
        self.data.add_entry(affector_spec, affectee_spec);
        self.anonymous_by_item
            .add_entry(item_uid, (affector_attr_rid, affectee_attr_rid));
    }
    pub(crate) fn add_with_source(
        &mut self,
        source_espec: EffectSpec,
        affector_aspec: AttrSpec,
        affectee_aspec: AttrSpec,
    ) {
        self.data.add_entry(affector_aspec, affectee_aspec);
        self.by_source.add_entry(source_espec, (affector_aspec, affectee_aspec));
        self.source_by_item.add_entry(affector_aspec.item_uid, source_espec);
        self.source_by_item.add_entry(affectee_aspec.item_uid, source_espec);
    }
    pub(in crate::svc::calc) fn remove_by_source(&mut self, source_espec: &EffectSpec) {
        for (affector_spec, affectee_spec) in self.by_source.remove_key(source_espec) {
            self.data.remove_entry(affector_spec, &affectee_spec);
            self.source_by_item.remove_entry(affector_spec.item_uid, source_espec);
            self.source_by_item.remove_entry(affectee_spec.item_uid, source_espec);
        }
    }
    pub(in crate::svc::calc) fn remove_item(&mut self, item_uid: UItemId) {
        // Anonymous dependencies
        for (affector_attr_rid, affectee_attr_rid) in self.anonymous_by_item.remove_key(&item_uid) {
            let affector_spec = AttrSpec::new(item_uid, affector_attr_rid);
            let affectee_spec = AttrSpec::new(item_uid, affectee_attr_rid);
            self.data.remove_entry(affector_spec, &affectee_spec);
        }
        // Dependencies with source
        for source in self.source_by_item.remove_key(&item_uid) {
            for (affector_spec, affectee_spec) in self.by_source.remove_key(&source) {
                self.data.remove_entry(affector_spec, &affectee_spec);
            }
        }
    }
}
