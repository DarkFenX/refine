use crate::{nd::NEffectNeutKind, rd::REffect};

/// Items which will be included in neut stats.
#[derive(Copy, Clone)]
pub struct StatNeutItemKinds {
    pub module: bool,
    pub minion: bool,
    pub bomb: bool,
    pub side_effect: bool,
}
impl StatNeutItemKinds {
    /// Include all item types in neut stats.
    pub fn all_enabled() -> Self {
        Self {
            module: true,
            minion: true,
            bomb: true,
            side_effect: true,
        }
    }
    /// Exclude all item types from neut stats.
    pub fn all_disabled() -> Self {
        Self {
            module: false,
            minion: false,
            bomb: false,
            side_effect: false,
        }
    }
    pub(in crate::svc::vast) fn resolve(&self, r_effect: &REffect) -> bool {
        let neut_kind = match &r_effect.neut {
            Some(neut) => neut.kind,
            None => return false,
        };
        match neut_kind {
            NEffectNeutKind::Module => self.module,
            NEffectNeutKind::Minion => self.minion,
            NEffectNeutKind::Bomb => self.bomb,
            NEffectNeutKind::SideEffect => self.side_effect,
        }
    }
}
