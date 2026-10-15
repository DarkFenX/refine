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
    pub(super) time: PValue,
    pub(super) data: CapSimEventData,
}
impl CapSimEvent {
    // Times are not negative, so bits of their values interpreted as integers are ordered the same
    // way as values, while being cheaper to compare. Adding zero ensures -0.0 becomes +0.0
    fn get_time_key(&self) -> u64 {
        (self.time.into_f64() + 0.0).to_bits()
    }
}
impl PartialOrd for CapSimEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CapSimEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Since sim is using max-heap, adjust parameters so that:
        // - events which have lower time are processed earlier
        // - with equal time, order of processing:
        //   - cycle check events
        //   - injector ready events, from highest injection to lowest, then from fastest cycle to
        //     slowest
        //   - cap gain events, from highest to lowest
        match other.get_time_key().cmp(&self.get_time_key()) {
            Ordering::Equal => match (&self.data, &other.data) {
                (CapSimEventData::CycleCheck(..), CapSimEventData::CycleCheck(..)) => Ordering::Equal,
                (CapSimEventData::CycleCheck(..), _) => Ordering::Greater,
                (CapSimEventData::InjectorReady(e1), CapSimEventData::InjectorReady(e2)) => e1.cmp_priority(e2),
                (CapSimEventData::InjectorReady(..), _) => Ordering::Greater,
                (CapSimEventData::CapChange(e1), CapSimEventData::CapChange(e2)) => {
                    match (e1.direction, e2.direction) {
                        (Direction::Gain, Direction::Gain) => e1.amount.cmp(&e2.amount),
                        (Direction::Loss, Direction::Loss) => e2.amount.cmp(&e1.amount),
                        (Direction::Gain, Direction::Loss) => Ordering::Greater,
                        (Direction::Loss, Direction::Gain) => Ordering::Less,
                    }
                }
                (CapSimEventData::CapChange(..), _) => Ordering::Less,
            },
            result => result,
        }
    }
}
impl PartialEq<Self> for CapSimEvent {
    fn eq(&self, other: &Self) -> bool {
        if self.get_time_key() != other.get_time_key() {
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
