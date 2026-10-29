use crate::{
    misc::{AttrSpec, EffectSpec},
    ud::UItemId,
    util::RMapRSet,
};

/// Intended to hold ad-hoc dependencies between attributes, which are not covered by registers
/// which hold data about regular modifiers.
#[derive(Clone)]
pub(in crate::svc::calc) struct DependencyRegister {
    // Map<affector spec, affectee specs> - this map could be StMapSetL2 with EffectSpec as source in
    // 3rd generic parameter, just to process collisions - i.e. when the same affector attribute and
    // affectee attribute are used by dependencies with different sources. But in our case it's not
    // needed with some external guarantees:
    // - when a modification source is removed, it clears dependent attribute values
    // - dependencies are re-established on every dependent recalculation
    // So, if effect source is removed (e.g. disabled via force stop mode), it clears up affectee
    // attribute, and when requested next time - remaining sources will re-add their dependencies,
    // allowing the affectee attribute to be cleared whenever linked attribute changes its value.
    data: RMapRSet<AttrSpec, AttrSpec>,
    // Map<source, (affector spec, affectee spec)>
    by_source: RMapRSet<EffectSpec, (AttrSpec, AttrSpec)>,
    // Map<item ID, sources>
    source_by_item: RMapRSet<UItemId, EffectSpec>,
}
impl DependencyRegister {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            data: RMapRSet::new(),
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
    pub(in crate::svc::calc) fn add_with_source(
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
        for source in self.source_by_item.remove_key(&item_uid) {
            for (affector_spec, affectee_spec) in self.by_source.remove_key(&source) {
                self.data.remove_entry(affector_spec, &affectee_spec);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl DependencyRegister {
        pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self {
                data,
                by_source,
                source_by_item,
            } = self;
            for (affector_spec, affector_data) in data.iter() {
                affector_spec.consistency_check(u_data, true)?;
                for affectee_spec in affector_data {
                    affectee_spec.consistency_check(u_data, true)?;
                }
            }
            for (source, specs) in by_source.iter() {
                source.consistency_check(u_data, true)?;
                for (affector_spec, affectee_spec) in specs {
                    affector_spec.consistency_check(u_data, true)?;
                    affectee_spec.consistency_check(u_data, true)?;
                }
            }
            for (&item_uid, sources) in source_by_item.iter() {
                item_uid.consistency_check(u_data, true)?;
                for source in sources {
                    source.consistency_check(u_data, true)?;
                }
            }
            Ok(())
        }
    }
}
