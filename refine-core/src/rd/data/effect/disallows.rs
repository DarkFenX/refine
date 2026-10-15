use bitflags::bitflags;

use crate::nd::NEffect;

bitflags! {
    /// Actions which are disallowed for the ship while an effect is running.
    pub(crate) struct REffectDisallows: u8 {
        const CLOAK = 1 << 0;
        const WARP = 1 << 1;
        const JUMP_GATE = 1 << 2;
        const JUMP_WH = 1 << 3;
        const JUMP_DRIVE = 1 << 4;
        const DOCK = 1 << 5;
        const TETHER = 1 << 6;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectDisallows {
    pub(super) fn from_n_effect(n_effect: &NEffect) -> Self {
        let mut disallows = Self::empty();
        disallows.set(Self::CLOAK, n_effect.disallows_cloak.is_some());
        disallows.set(Self::WARP, n_effect.disallows_warp.is_some());
        disallows.set(Self::JUMP_GATE, n_effect.disallows_jump_gate.is_some());
        disallows.set(Self::JUMP_WH, n_effect.disallows_jump_wh.is_some());
        disallows.set(Self::JUMP_DRIVE, n_effect.disallows_jump_drive.is_some());
        disallows.set(Self::DOCK, n_effect.disallows_dock.is_some());
        disallows.set(Self::TETHER, n_effect.disallows_tether.is_some());
        disallows
    }
}
