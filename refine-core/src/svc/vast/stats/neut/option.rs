use crate::{misc::DefOption, nd::NEffectNeutKind, rd::REffect};

/// Items which will be included in neut stats.
#[derive(Copy, Clone)]
pub struct StatNeutItemKinds {
    pub default: bool = true,
    pub module: DefOption = DefOption::Default,
    pub minion: DefOption = DefOption::Default,
    pub bomb: DefOption = DefOption::Default,
    pub side_effect: DefOption = DefOption::Default,
}
impl StatNeutItemKinds {
    pub(in crate::svc::vast) fn resolve(&self, r_effect: &REffect) -> bool {
        let neut_kind = match &r_effect.neut {
            Some(neut) => neut.kind,
            None => return false,
        };
        match neut_kind {
            NEffectNeutKind::Module => self.module.is_enabled(self.default),
            NEffectNeutKind::Minion => self.minion.is_enabled(self.default),
            NEffectNeutKind::Bomb => self.bomb.is_enabled(self.default),
            NEffectNeutKind::SideEffect => self.side_effect.is_enabled(self.default),
        }
    }
}
