use crate::{
    ad::AEffectId,
    misc::EffectMode,
    rd::{RData, REffectId},
    util::RMap,
};

pub(in crate::ud::item) type UEffectModeOverrideIter<'a> = impl ExactSizeIterator<Item = (AEffectId, EffectMode)>;

#[derive(Clone)]
pub(in crate::ud::item) struct UEffectModes {
    by_aid: RMap<AEffectId, EffectMode>,
    by_rid: RMap<REffectId, EffectMode>,
}
impl UEffectModes {
    pub(in crate::ud::item) fn new() -> Self {
        Self {
            by_aid: RMap::new(),
            by_rid: RMap::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Query methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UEffectModes {
    pub(in crate::ud::item) fn get_by_rid(&self, effect_rid: &REffectId) -> EffectMode {
        match self.by_rid.get(effect_rid) {
            Some(effect_mode) => *effect_mode,
            None => EffectMode::default(),
        }
    }
    pub(in crate::ud::item) fn get_by_aid(&self, effect_aid: &AEffectId) -> EffectMode {
        match self.by_aid.get(effect_aid) {
            Some(effect_mode) => *effect_mode,
            None => EffectMode::default(),
        }
    }
    #[define_opaque(UEffectModeOverrideIter)]
    pub(in crate::ud::item) fn iter_overrides_with_aids(&self) -> UEffectModeOverrideIter<'_> {
        self.by_aid
            .iter()
            .map(|(&effect_aid, &effect_mode)| (effect_aid, effect_mode))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Modification methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UEffectModes {
    /// Returns old effect mode
    pub(in crate::ud::item) fn set_by_aid(
        &mut self,
        effect_aid: AEffectId,
        effect_mode: EffectMode,
        r_data: &RData,
    ) -> EffectMode {
        let old_effect_mode = match effect_mode == EffectMode::default() {
            true => {
                let old_effect_mode = self.by_aid.remove(&effect_aid);
                if let Some(effect_rid) = r_data.get_effect_rid_by_aid(&effect_aid) {
                    self.by_rid.remove(&effect_rid);
                }
                old_effect_mode
            }
            false => {
                let old_effect_mode = self.by_aid.insert(effect_aid, effect_mode);
                if let Some(effect_rid) = r_data.get_effect_rid_by_aid(&effect_aid) {
                    self.by_rid.insert(effect_rid, effect_mode);
                }
                old_effect_mode
            }
        };
        old_effect_mode.unwrap_or_default()
    }
    pub(in crate::ud::item) fn update_rids(&mut self, r_data: &RData) {
        self.by_rid.clear();
        for (effect_aid, effect_mode) in self.by_aid.iter() {
            if let Some(effect_rid) = r_data.get_effect_rid_by_aid(effect_aid) {
                self.by_rid.insert(effect_rid, *effect_mode);
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

    impl UEffectModes {
        pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self {
                by_rid,
                // Not checked
                by_aid: _,
            } = self;
            for effect_rid in by_rid.keys() {
                effect_rid.consistency_check(u_data)?;
            }
            Ok(())
        }
    }
}
