use crate::def::Count;

pub(super) enum AggrBreacherTicks {
    LimitedSimple(AbtLimitedSimple),
    LimitedComplex(AbtLimitedComplex),
    InfiniteSimple(AbtInfiniteSimple),
    InfiniteComplex1(AbtInfiniteComplex1),
    InfiniteComplex2(AbtInfiniteComplex2),
    LoopedSimple(AbtLoopedSimple),
    LoopedComplex(AbtLoopedComplex),
}
impl AggrBreacherTicks {
    pub(super) fn get_looped_part(&self) -> Option<AggrBreacherTicksLooped> {
        match &self {
            Self::LimitedSimple(inner) => inner.get_looped_part(),
            Self::LimitedComplex(inner) => inner.get_looped_part(),
            Self::InfiniteSimple(inner) => inner.get_looped_part(),
            Self::InfiniteComplex1(inner) => inner.get_looped_part(),
            Self::InfiniteComplex2(inner) => inner.get_looped_part(),
            Self::LoopedSimple(inner) => inner.get_looped_part(),
            Self::LoopedComplex(inner) => inner.get_looped_part(),
        }
    }
    pub(super) fn is_applied_on_tick(&self, tick: Count) -> bool {
        match &self {
            Self::LimitedSimple(inner) => inner.is_applied_on_tick(tick),
            Self::LimitedComplex(inner) => inner.is_applied_on_tick(tick),
            Self::InfiniteSimple(inner) => inner.is_applied_on_tick(tick),
            Self::InfiniteComplex1(inner) => inner.is_applied_on_tick(tick),
            Self::InfiniteComplex2(inner) => inner.is_applied_on_tick(tick),
            Self::LoopedSimple(inner) => inner.is_applied_on_tick(tick),
            Self::LoopedComplex(inner) => inner.is_applied_on_tick(tick),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum AggrBreacherTicksLooped {
    InfiniteSimple(AbtInfiniteSimple),
    LoopedSimple(AbtLoopedSimple),
    LoopedComplex(AbtLoopedComplex),
}
impl AggrBreacherTicksLooped {
    pub(super) fn get_loop_len(&self) -> Count {
        match &self {
            Self::InfiniteSimple(inner) => inner.get_loop_len(),
            Self::LoopedSimple(inner) => inner.get_loop_len(),
            Self::LoopedComplex(inner) => inner.get_loop_len(),
        }
    }
    pub(super) fn is_applied_on_tick(&self, tick: Count) -> bool {
        match &self {
            Self::InfiniteSimple(inner) => inner.is_applied_on_tick(tick),
            Self::LoopedSimple(inner) => inner.is_applied_on_tick(tick),
            Self::LoopedComplex(inner) => inner.is_applied_on_tick(tick),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLimitedSimple {
    pub(super) count: Count,
}
impl AbtLimitedSimple {
    fn get_looped_part(&self) -> Option<AggrBreacherTicksLooped> {
        None
    }
    fn is_applied_on_tick(&self, tick: Count) -> bool {
        tick < self.count
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtLimitedComplex {
    pub(super) dmg_tick_count: Count,
    pub(super) inactive_tick_count: Count,
    pub(super) repeat_count: Count,
}
impl AbtLimitedComplex {
    fn get_looped_part(&self) -> Option<AggrBreacherTicksLooped> {
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
pub(super) struct AbtInfiniteSimple {}
impl AbtInfiniteSimple {
    fn get_looped_part(&self) -> Option<AggrBreacherTicksLooped> {
        Some(AggrBreacherTicksLooped::InfiniteSimple(*self))
    }
    fn get_loop_len(&self) -> Count {
        1
    }
    fn is_applied_on_tick(&self, _tick: Count) -> bool {
        true
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtInfiniteComplex1 {
    pub(super) p1_dmg_tick_count: Count,
    pub(super) p1_inactive_tick_count: Count,
    pub(super) p1_repeat_count: Count,
    pub(super) p2_dmg_tick_count: Count,
    pub(super) p2_inactive_tick_count: Count,
}
impl AbtInfiniteComplex1 {
    fn get_looped_part(&self) -> Option<AggrBreacherTicksLooped> {
        match self.p2_inactive_tick_count {
            0 => Some(AggrBreacherTicksLooped::InfiniteSimple(AbtInfiniteSimple {})),
            _ => Some(AggrBreacherTicksLooped::LoopedSimple(AbtLoopedSimple {
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
pub(super) struct AbtInfiniteComplex2 {
    pub(super) p1_dmg_tick_count: Count,
    pub(super) p1_inactive_tick_count: Count,
    pub(super) p1_repeat_count: Count,
    pub(super) p2_dmg_tick_count: Count,
    pub(super) p2_inactive_tick_count: Count,
    pub(super) p2_repeat_count: Count,
    pub(super) p3_dmg_tick_count: Count,
    pub(super) p3_inactive_tick_count: Count,
}
impl AbtInfiniteComplex2 {
    fn get_looped_part(&self) -> Option<AggrBreacherTicksLooped> {
        match self.p3_inactive_tick_count {
            0 => Some(AggrBreacherTicksLooped::InfiniteSimple(AbtInfiniteSimple {})),
            _ => Some(AggrBreacherTicksLooped::LoopedSimple(AbtLoopedSimple {
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
pub(super) struct AbtLoopedSimple {
    pub(super) dmg_tick_count: Count,
    pub(super) inactive_tick_count: Count,
}
impl AbtLoopedSimple {
    fn get_looped_part(&self) -> Option<AggrBreacherTicksLooped> {
        Some(AggrBreacherTicksLooped::LoopedSimple(*self))
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
pub(super) struct AbtLoopedComplex {
    pub(super) p1_dmg_tick_count: Count,
    pub(super) p1_inactive_tick_count: Count,
    pub(super) p1_repeat_count: Count,
    pub(super) p2_dmg_tick_count: Count,
    pub(super) p2_inactive_tick_count: Count,
    pub(super) p2_repeat_count: Count,
}
impl AbtLoopedComplex {
    fn get_looped_part(&self) -> Option<AggrBreacherTicksLooped> {
        Some(AggrBreacherTicksLooped::LoopedComplex(*self))
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
