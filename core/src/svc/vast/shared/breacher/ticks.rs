use crate::def::DefCount;

// In designations, first letter:
// - L - limited
// - I - infinite
// Second letter:
// - S - simple - single damage tick
// - C - complex - set count of damage ticks + set count of ticks without damage

pub(super) enum AggrBreacherTicks {
    Ls(AbtLs),
    Lc(AbtLc),
    Is(AbtIs),
    Ic(AbtIc),
    LcIc(AbtLcIc),
    LcLcIc(AbtLcLcIc),
    LoopLcLc(AbtLoopLcLc),
}
impl AggrBreacherTicks {
    pub(super) fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        match &self {
            Self::Ls(inner) => inner.get_loop(),
            Self::Lc(inner) => inner.get_loop(),
            Self::Is(inner) => inner.get_loop(),
            Self::Ic(inner) => inner.get_loop(),
            Self::LcIc(inner) => inner.get_loop(),
            Self::LcLcIc(inner) => inner.get_loop(),
            Self::LoopLcLc(inner) => inner.get_loop(),
        }
    }
    pub(super) fn is_applied_on_tick(&self, tick: DefCount) -> bool {
        match &self {
            Self::Ls(inner) => inner.is_applied_on_tick(tick),
            Self::Lc(inner) => inner.is_applied_on_tick(tick),
            Self::Is(inner) => inner.is_applied_on_tick(tick),
            Self::Ic(inner) => inner.is_applied_on_tick(tick),
            Self::LcIc(inner) => inner.is_applied_on_tick(tick),
            Self::LcLcIc(inner) => inner.is_applied_on_tick(tick),
            Self::LoopLcLc(inner) => inner.is_applied_on_tick(tick),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum AggrBreacherTicksLooped {
    Is(AbtIs),
    Ic(AbtIc),
    LoopLcLc(AbtLoopLcLc),
}
impl AggrBreacherTicksLooped {
    pub(super) fn get_loop_len(&self) -> DefCount {
        match &self {
            Self::Is(inner) => inner.get_loop_len(),
            Self::Ic(inner) => inner.get_loop_len(),
            Self::LoopLcLc(inner) => inner.get_loop_len(),
        }
    }
    pub(super) fn is_applied_on_tick(&self, tick: DefCount) -> bool {
        match &self {
            Self::Is(inner) => inner.is_applied_on_tick(tick),
            Self::Ic(inner) => inner.is_applied_on_tick(tick),
            Self::LoopLcLc(inner) => inner.is_applied_on_tick(tick),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Enum element definitions
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLs {
    pub(super) count: DefCount,
}
impl AbtLs {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        None
    }
    fn is_applied_on_tick(&self, tick: DefCount) -> bool {
        tick < self.count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLc {
    pub(super) dmg_tick_count: DefCount,
    pub(super) tick_count: DefCount,
    pub(super) repeat_count: DefCount,
}
impl AbtLc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        None
    }
    fn is_applied_on_tick(&self, tick: DefCount) -> bool {
        if tick / self.tick_count >= self.repeat_count {
            return false;
        };
        let tick = tick % self.tick_count;
        tick < self.dmg_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtIs {}
impl AbtIs {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        Some(AggrBreacherTicksLooped::Is(*self))
    }
    fn get_loop_len(&self) -> DefCount {
        1
    }
    fn is_applied_on_tick(&self, _tick: DefCount) -> bool {
        true
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtIc {
    pub(super) dmg_tick_count: DefCount,
    pub(super) tick_count: DefCount,
}
impl AbtIc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        Some(AggrBreacherTicksLooped::Ic(*self))
    }
    fn get_loop_len(&self) -> DefCount {
        self.tick_count
    }
    fn is_applied_on_tick(&self, tick: DefCount) -> bool {
        let tick = tick % self.tick_count;
        tick < self.dmg_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLcIc {
    pub(super) p1_dmg_tick_count: DefCount,
    pub(super) p1_tick_count: DefCount,
    pub(super) p1_repeat_count: DefCount,
    pub(super) p2_dmg_tick_count: DefCount,
    pub(super) p2_tick_count: DefCount,
}
impl AbtLcIc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        match self.p2_dmg_tick_count >= self.p2_tick_count {
            true => Some(AggrBreacherTicksLooped::Is(AbtIs {})),
            false => Some(AggrBreacherTicksLooped::Ic(AbtIc {
                dmg_tick_count: self.p2_dmg_tick_count,
                tick_count: self.p2_tick_count,
            })),
        }
    }
    fn is_applied_on_tick(&self, tick: DefCount) -> bool {
        let p1_total_ticks = self.p1_tick_count * self.p1_repeat_count;
        if tick < p1_total_ticks {
            let tick = tick % self.p1_tick_count;
            return tick < self.p1_dmg_tick_count;
        }
        let tick = tick - p1_total_ticks;
        let tick = tick % self.p2_tick_count;
        tick < self.p2_dmg_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLcLcIc {
    pub(super) p1_dmg_tick_count: DefCount,
    pub(super) p1_tick_count: DefCount,
    pub(super) p1_repeat_count: DefCount,
    pub(super) p2_dmg_tick_count: DefCount,
    pub(super) p2_tick_count: DefCount,
    pub(super) p2_repeat_count: DefCount,
    pub(super) p3_dmg_tick_count: DefCount,
    pub(super) p3_tick_count: DefCount,
}
impl AbtLcLcIc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        match self.p3_dmg_tick_count >= self.p3_tick_count {
            true => Some(AggrBreacherTicksLooped::Is(AbtIs {})),
            false => Some(AggrBreacherTicksLooped::Ic(AbtIc {
                dmg_tick_count: self.p3_dmg_tick_count,
                tick_count: self.p3_tick_count,
            })),
        }
    }
    fn is_applied_on_tick(&self, tick: DefCount) -> bool {
        let p1_total_ticks = self.p1_tick_count * self.p1_repeat_count;
        if tick < p1_total_ticks {
            let tick = tick % self.p1_tick_count;
            return tick < self.p1_dmg_tick_count;
        }
        let tick = tick - p1_total_ticks;
        let p2_total_ticks = self.p2_tick_count * self.p2_repeat_count;
        if tick < p2_total_ticks {
            let tick = tick % self.p2_tick_count;
            return tick < self.p2_dmg_tick_count;
        }
        let tick = tick - p2_total_ticks;
        let tick = tick % self.p3_tick_count;
        tick < self.p3_dmg_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLoopLcLc {
    pub(super) p1_dmg_tick_count: DefCount,
    pub(super) p1_tick_count: DefCount,
    pub(super) p1_repeat_count: DefCount,
    pub(super) p2_dmg_tick_count: DefCount,
    pub(super) p2_tick_count: DefCount,
    pub(super) p2_repeat_count: DefCount,
}
impl AbtLoopLcLc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        Some(AggrBreacherTicksLooped::LoopLcLc(*self))
    }
    fn get_loop_len(&self) -> DefCount {
        self.p1_tick_count * self.p1_repeat_count + self.p2_tick_count * self.p2_repeat_count
    }
    fn is_applied_on_tick(&self, tick: DefCount) -> bool {
        let p1_total_ticks = self.p1_tick_count * self.p1_repeat_count;
        let p2_total_ticks = self.p2_tick_count * self.p2_repeat_count;
        let total_ticks = p1_total_ticks + p2_total_ticks;
        let tick = tick % total_ticks;
        if tick < p1_total_ticks {
            let tick = tick % self.p1_tick_count;
            return tick < self.p1_dmg_tick_count;
        }
        let tick = (tick - p1_total_ticks) % self.p2_tick_count;
        tick < self.p2_dmg_tick_count
    }
}
