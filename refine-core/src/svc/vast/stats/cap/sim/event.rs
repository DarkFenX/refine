use std::cmp::Ordering;

use crate::{
    PValue,
    svc::vast::{aggr::AggrIter, stats::cap::sim::shared::Direction},
    util::PrefetchPeekable,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event main type
////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering key is moved out of boxes to allow faster access
pub(super) struct CapSimEvent {
    // Events are ordered by this key, except for injector events with equal key
    key: CapSimEventKey,
    pub(super) data: CapSimEventData,
}
impl CapSimEvent {
    pub(super) fn new(time: PValue, data: CapSimEventData) -> Self {
        Self {
            key: CapSimEventKey::new(time, &data),
            data,
        }
    }
    pub(super) fn get_time(&self) -> PValue {
        self.key.get_time()
    }
    pub(super) fn set_time(&mut self, time: PValue) {
        self.key = self.key.with_time(time);
    }
    #[inline(always)]
    fn is_injector_tie(&self, other: &Self) -> bool {
        // Non-short-circuiting operator is used, since key equality is unpredictable, while
        // injector ties are almost non-existent
        self.key.is_injector() & (self.key == other.key)
    }
    #[cold]
    #[inline(never)]
    fn cmp_injectors(&self, other: &Self) -> Ordering {
        match (&self.data, &other.data) {
            (CapSimEventData::InjectorReady(e1), CapSimEventData::InjectorReady(e2)) => e1.cmp_priority(e2),
            _ => Ordering::Equal,
        }
    }
}
impl PartialOrd for CapSimEvent {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    // Comparison operators are used by binary heap. They compare keys directly, which is cheaper
    // than producing an ordering first
    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        match self.is_injector_tie(other) {
            true => self.cmp_injectors(other).is_lt(),
            false => self.key < other.key,
        }
    }
    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        match self.is_injector_tie(other) {
            true => self.cmp_injectors(other).is_le(),
            false => self.key <= other.key,
        }
    }
    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        match self.is_injector_tie(other) {
            true => self.cmp_injectors(other).is_gt(),
            false => self.key > other.key,
        }
    }
    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        match self.is_injector_tie(other) {
            true => self.cmp_injectors(other).is_ge(),
            false => self.key >= other.key,
        }
    }
}
impl Ord for CapSimEvent {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.key.cmp(&other.key) {
            Ordering::Equal if self.key.is_injector() => self.cmp_injectors(other),
            result => result,
        }
    }
}
impl PartialEq<Self> for CapSimEvent {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for CapSimEvent {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event ordering key
////////////////////////////////////////////////////////////////////////////////////////////////////
// With equal time, order of processing:
// - cycle check events
// - injector ready events, from highest injection to lowest, then from fastest cycle to slowest
// - cap gain events, from highest to lowest
// - cap loss events, from lowest to highest
const PRIO_CYCLE_CHECK: u64 = u64::MAX;
const PRIO_INJECTOR: u64 = u64::MAX - 1;
// Bits of non-negative non-NaN floats take up to 63 bits, so gains are always above losses
const PRIO_GAIN_BASE: u64 = 1 << 63;
const PRIO_LOSS_BASE: u64 = (1 << 63) - 1;

/// Higher 64 bits contain inverted time bits, since sim is using max-heap, and events which have
/// lower time are processed earlier. Lower 64 bits contain processing priority of events with equal
/// time.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct CapSimEventKey(u128);
impl CapSimEventKey {
    fn new(time: PValue, data: &CapSimEventData) -> Self {
        let prio = match data {
            CapSimEventData::CycleCheck(..) => PRIO_CYCLE_CHECK,
            CapSimEventData::InjectorReady(..) => PRIO_INJECTOR,
            CapSimEventData::CapChange(event) => match event.direction {
                Direction::Gain => PRIO_GAIN_BASE + get_bits(event.amount),
                Direction::Loss => PRIO_LOSS_BASE - get_bits(event.amount),
            },
        };
        Self(((!get_bits(time) as u128) << 64) | prio as u128)
    }
    fn with_time(self, time: PValue) -> Self {
        Self(((!get_bits(time) as u128) << 64) | (self.0 as u64) as u128)
    }
    fn get_time(self) -> PValue {
        PValue::from_f64_unchecked(f64::from_bits(!((self.0 >> 64) as u64)))
    }
    fn is_injector(self) -> bool {
        self.0 as u64 == PRIO_INJECTOR
    }
}

// Values are not negative, so bits of their values interpreted as integers are ordered the same way
// as values. Adding zero ensures -0.0 becomes +0.0
fn get_bits(value: PValue) -> u64 {
    (value.into_f64() + 0.0).to_bits()
}

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
