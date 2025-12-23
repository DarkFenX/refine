use crate::def::Count;

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
    pub(super) fn is_applied_on_tick(&self, tick: Count) -> bool {
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
    pub(super) fn get_loop_len(&self) -> Count {
        match &self {
            Self::Is(inner) => inner.get_loop_len(),
            Self::Ic(inner) => inner.get_loop_len(),
            Self::LoopLcLc(inner) => inner.get_loop_len(),
        }
    }
    pub(super) fn is_applied_on_tick(&self, tick: Count) -> bool {
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
    pub(super) count: Count,
}
impl AbtLs {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        None
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        tick < self.count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLc {
    pub(super) dmg_tick_count: Count,
    pub(super) inactive_tick_count: Count,
    pub(super) repeat_count: Count,
}
impl AbtLc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        None
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        let ticks_per_cycle = self.dmg_tick_count + self.inactive_tick_count;
        if tick / ticks_per_cycle >= self.repeat_count {
            return false;
        };
        let in_cycle_tick = tick % ticks_per_cycle;
        in_cycle_tick < self.dmg_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtIs {}
impl AbtIs {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        Some(AggrBreacherTicksLooped::Is(*self))
    }
    fn get_loop_len(&self) -> Count {
        1
    }
    fn is_applied_on_tick(&self, _tick: Count) -> bool {
        true
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtIc {
    pub(super) dmg_tick_count: Count,
    pub(super) inactive_tick_count: Count,
}
impl AbtIc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        Some(AggrBreacherTicksLooped::Ic(*self))
    }
    fn get_loop_len(&self) -> Count {
        self.dmg_tick_count + self.inactive_tick_count
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        let ticks_per_cycle = self.dmg_tick_count + self.inactive_tick_count;
        let in_cycle_tick = tick % ticks_per_cycle;
        in_cycle_tick < self.dmg_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLcIc {
    pub(super) p1_dmg_tick_count: Count,
    pub(super) p1_inactive_tick_count: Count,
    pub(super) p1_repeat_count: Count,
    pub(super) p2_dmg_tick_count: Count,
    pub(super) p2_inactive_tick_count: Count,
}
impl AbtLcIc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        match self.p2_inactive_tick_count {
            0 => Some(AggrBreacherTicksLooped::Is(AbtIs {})),
            _ => Some(AggrBreacherTicksLooped::Ic(AbtIc {
                dmg_tick_count: self.p2_dmg_tick_count,
                inactive_tick_count: self.p2_inactive_tick_count,
            })),
        }
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        let p1_inner_ticks = self.p1_dmg_tick_count + self.p1_inactive_tick_count;
        let p1_total_ticks = p1_inner_ticks * self.p1_repeat_count;
        if tick < p1_total_ticks {
            let tick = tick % p1_inner_ticks;
            return tick < self.p1_dmg_tick_count;
        }
        let tick = tick - p1_total_ticks;
        let p2_inner_ticks = self.p2_dmg_tick_count + self.p2_inactive_tick_count;
        let tick = tick % p2_inner_ticks;
        tick < self.p2_dmg_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLcLcIc {
    pub(super) p1_dmg_tick_count: Count,
    pub(super) p1_inactive_tick_count: Count,
    pub(super) p1_repeat_count: Count,
    pub(super) p2_dmg_tick_count: Count,
    pub(super) p2_inactive_tick_count: Count,
    pub(super) p2_repeat_count: Count,
    pub(super) p3_dmg_tick_count: Count,
    pub(super) p3_inactive_tick_count: Count,
}
impl AbtLcLcIc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        match self.p3_inactive_tick_count {
            0 => Some(AggrBreacherTicksLooped::Is(AbtIs {})),
            _ => Some(AggrBreacherTicksLooped::Ic(AbtIc {
                dmg_tick_count: self.p3_dmg_tick_count,
                inactive_tick_count: self.p3_inactive_tick_count,
            })),
        }
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        let p1_inner_ticks = self.p1_dmg_tick_count + self.p1_inactive_tick_count;
        let p1_total_ticks = p1_inner_ticks * self.p1_repeat_count;
        if tick < p1_total_ticks {
            let tick = tick % p1_inner_ticks;
            return tick < self.p1_dmg_tick_count;
        }
        let tick = tick - p1_total_ticks;
        let p2_inner_ticks = self.p2_dmg_tick_count + self.p2_inactive_tick_count;
        let p2_total_ticks = p2_inner_ticks * self.p2_repeat_count;
        if tick < p2_total_ticks {
            let tick = tick % p2_inner_ticks;
            return tick < self.p2_dmg_tick_count;
        }
        let tick = tick - p2_total_ticks;
        let p3_inner_ticks = self.p3_dmg_tick_count + self.p3_inactive_tick_count;
        let tick = tick % p3_inner_ticks;
        tick < self.p3_dmg_tick_count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLoopLcLc {
    pub(super) p1_dmg_tick_count: Count,
    pub(super) p1_inactive_tick_count: Count,
    pub(super) p1_repeat_count: Count,
    pub(super) p2_dmg_tick_count: Count,
    pub(super) p2_inactive_tick_count: Count,
    pub(super) p2_repeat_count: Count,
}
impl AbtLoopLcLc {
    fn get_loop(&self) -> Option<AggrBreacherTicksLooped> {
        Some(AggrBreacherTicksLooped::LoopLcLc(*self))
    }
    fn get_loop_len(&self) -> Count {
        (self.p1_dmg_tick_count + self.p1_inactive_tick_count) * self.p1_repeat_count
            + (self.p2_dmg_tick_count + self.p2_inactive_tick_count) * self.p2_repeat_count
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        let p1_inner_ticks = self.p1_dmg_tick_count + self.p1_inactive_tick_count;
        let p1_total_ticks = p1_inner_ticks * self.p1_repeat_count;
        let p2_inner_ticks = self.p2_dmg_tick_count + self.p2_inactive_tick_count;
        let p2_total_ticks = p2_inner_ticks * self.p2_repeat_count;
        let total_ticks = p1_total_ticks + p2_total_ticks;
        let tick = tick % total_ticks;
        if tick < p1_total_ticks {
            let tick = tick % p1_inner_ticks;
            return tick < self.p1_dmg_tick_count;
        }
        let tick = (tick - p1_total_ticks) % p2_inner_ticks;
        tick < self.p2_dmg_tick_count
    }
}
