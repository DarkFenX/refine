use crate::{def::ItemKey, misc::EffectSpec, uad::UadProjRange, util::RMap};

// Holds info about effect projections
#[derive(Clone)]
pub(crate) struct EProjs {
    pub(super) ranges: RMap<(EffectSpec, ItemKey), UadProjRange>,
}
impl EProjs {
    pub(in crate::svc) fn new() -> Self {
        Self { ranges: RMap::new() }
    }
    // Query methods
    pub(in crate::svc) fn get_range(
        &self,
        projector_espec: EffectSpec,
        projectee_item_key: ItemKey,
    ) -> Option<UadProjRange> {
        self.ranges.get(&(projector_espec, projectee_item_key)).copied()
    }
    // Modification methods
    pub(in crate::svc) fn add_range(
        &mut self,
        projector_espec: EffectSpec,
        projectee_item_key: ItemKey,
        range: Option<UadProjRange>,
    ) {
        if let Some(range) = range {
            self.ranges.insert((projector_espec, projectee_item_key), range);
        }
    }
    pub(in crate::svc) fn change_range(
        &mut self,
        projector_espec: EffectSpec,
        projectee_item_key: ItemKey,
        range: Option<UadProjRange>,
    ) {
        match range {
            Some(range) => self.ranges.insert((projector_espec, projectee_item_key), range),
            None => self.ranges.remove(&(projector_espec, projectee_item_key)),
        };
    }
    pub(in crate::svc) fn remove_range(&mut self, affector_espec: EffectSpec, affectee_item_key: ItemKey) {
        self.ranges.remove(&(affector_espec, affectee_item_key));
    }
}
