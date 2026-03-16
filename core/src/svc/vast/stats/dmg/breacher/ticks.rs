use crate::num::Count;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum AggrBreacherTicks {
    Infinite(AbtInfinite),
    CycleSimple(AbtCycleSimple),
    CycleComplex(AbtCycleComplex),
}
impl AggrBreacherTicks {
    pub(super) fn get_initial_delay(&self) -> Count {
        match self {
            Self::Infinite(inner) => inner.get_initial_delay(),
            Self::CycleSimple(inner) => inner.get_initial_delay(),
            Self::CycleComplex(inner) => inner.get_initial_delay(),
        }
    }
    pub(super) fn get_loop_len(&self) -> Count {
        match self {
            Self::Infinite(inner) => inner.get_loop_len(),
            Self::CycleSimple(inner) => inner.get_loop_len(),
            Self::CycleComplex(inner) => inner.get_loop_len(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle: infinite damage ticks
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtInfinite {
    pub(super) initial_delay: Count,
}
impl AbtInfinite {
    fn get_initial_delay(&self) -> Count {
        self.initial_delay
    }
    fn get_loop_len(&self) -> Count {
        Count::ONE
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle:
// - damage ticks
// - gap ticks
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtCycleSimple {
    pub(super) initial_delay: Count,
    pub(super) cycle_dmg: Count,
    pub(super) cycle_total: Count,
}
impl AbtCycleSimple {
    fn get_initial_delay(&self) -> Count {
        self.initial_delay
    }
    fn get_loop_len(&self) -> Count {
        self.cycle_total
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle:
// - repeated N times:
//   - damage ticks
//   - gap ticks
// - damage ticks
// - gap ticks
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct AbtCycleComplex {
    pub(super) initial_delay: Count,
    pub(super) cycle_p1_dmg: Count,
    pub(super) cycle_p1_total: Count,
    pub(super) cycle_p1_repeats: Count,
    pub(super) cycle_p2_dmg: Count,
    pub(super) cycle_p2_total: Count,
}
impl AbtCycleComplex {
    fn get_initial_delay(&self) -> Count {
        self.initial_delay
    }
    fn get_loop_len(&self) -> Count {
        self.cycle_p1_total * self.cycle_p1_repeats + self.cycle_p2_total
    }
}
