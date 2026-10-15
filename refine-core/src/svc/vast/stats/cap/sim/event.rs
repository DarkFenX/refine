use std::cmp::Ordering;

use crate::{
    PValue,
    svc::vast::{aggr::AggrIter, stats::cap::sim::shared::Direction},
    util::PrefetchPeekable,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event main type
////////////////////////////////////////////////////////////////////////////////////////////////////
// Time is moved out of boxes to allow faster access
pub(super) struct CapSimEvent {
    // Times are not negative, so bits of their values interpreted as integers are ordered the same
    // way as values, while being cheaper to compare
    time_key: u64,
    pub(super) data: CapSimEventData,
}
impl CapSimEvent {
    pub(super) fn new(time: PValue, data: CapSimEventData) -> Self {
        Self {
            // Adding zero ensures -0.0 becomes +0.0
            time_key: (time.into_f64() + 0.0).to_bits(),
            data,
        }
    }
    pub(super) fn get_time(&self) -> PValue {
        PValue::from_f64_unchecked(f64::from_bits(self.time_key))
    }
    // With equal time, order of processing:
    // - cycle check events
    // - injector ready events, from highest injection to lowest, then from fastest cycle to slowest
    // - cap gain events, from highest to lowest
    #[inline(never)]
    fn cmp_same_time(&self, other: &Self) -> Ordering {
        match (&self.data, &other.data) {
            (CapSimEventData::CycleCheck(..), CapSimEventData::CycleCheck(..)) => Ordering::Equal,
            (CapSimEventData::CycleCheck(..), _) => Ordering::Greater,
            (CapSimEventData::InjectorReady(e1), CapSimEventData::InjectorReady(e2)) => e1.cmp_priority(e2),
            (CapSimEventData::InjectorReady(..), _) => Ordering::Greater,
            (CapSimEventData::CapChange(e1), CapSimEventData::CapChange(e2)) => match (e1.direction, e2.direction) {
                (Direction::Gain, Direction::Gain) => e1.amount.cmp(&e2.amount),
                (Direction::Loss, Direction::Loss) => e2.amount.cmp(&e1.amount),
                (Direction::Gain, Direction::Loss) => Ordering::Greater,
                (Direction::Loss, Direction::Gain) => Ordering::Less,
            },
            (CapSimEventData::CapChange(..), _) => Ordering::Less,
        }
    }
}
impl PartialOrd for CapSimEvent {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CapSimEvent {
    // Comparison by time is inlined, since it is what decides order in absolute majority of cases
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        // Since sim is using max-heap, events which have lower time are processed earlier
        match other.time_key.cmp(&self.time_key) {
            Ordering::Equal => self.cmp_same_time(other),
            result => result,
        }
    }
}
impl PartialEq<Self> for CapSimEvent {
    fn eq(&self, other: &Self) -> bool {
        if self.time_key != other.time_key {
            return false;
        }
        match (&self.data, &other.data) {
            (CapSimEventData::CycleCheck(..), CapSimEventData::CycleCheck(..)) => true,
            (CapSimEventData::InjectorReady(e1), CapSimEventData::InjectorReady(e2)) => {
                e1.cmp_priority(e2) == Ordering::Equal
            }
            (CapSimEventData::CapChange(e1), CapSimEventData::CapChange(e2)) => e1.amount.eq(&e2.amount),
            _ => false,
        }
    }
}
impl Eq for CapSimEvent {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event enum and its variants
////////////////////////////////////////////////////////////////////////////////////////////////////
// Boxes to minimize type size, to allow binary heap move it faster
pub(super) enum CapSimEventData {
    CycleCheck(Box<CapSimEventCycleCheck>),
    InjectorReady(Box<CapSimEventInjector>),
    CapChange(CapSimEventCapChange),
}

pub(super) struct CapSimEventCycleCheck {
    pub(super) cycle_iter: AggrIter<PValue>,
    pub(super) direction: Direction,
}

pub(super) struct CapSimEventInjector {
    pub(super) cycle_iter: PrefetchPeekable<AggrIter<PValue>>,
}
impl CapSimEventInjector {
    // Injectors which inject more are processed first, then injectors with faster cycle
    fn cmp_priority(&self, other: &Self) -> Ordering {
        self.get_immediate_instance()
            .unwrap_or(PValue::ZERO)
            .cmp(&other.get_immediate_instance().unwrap_or(PValue::ZERO))
            .then_with(|| other.get_cycle_duration().cmp(&self.get_cycle_duration()))
    }
    fn get_cycle_duration(&self) -> Option<PValue> {
        self.cycle_iter
            .peek()
            .map(|cycle_iter_item| cycle_iter_item.cycle_duration)
    }
    pub(super) fn get_immediate_instance(&self) -> Option<PValue> {
        self.cycle_iter
            .peek()
            .and_then(|cycle_iter_item| cycle_iter_item.output.get_immediate_instance())
    }
}

pub(super) struct CapSimEventCapChange {
    pub(super) amount: PValue,
    pub(super) direction: Direction,
}
