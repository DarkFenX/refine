use std::cmp::Ordering;

use crate::{
    PValue,
    svc::{
        cycle::CycleSeq,
        output::Output,
        vast::{
            aggr::{AggrIter, AggrIterData},
            stats::cap::sim::shared::Direction,
        },
    },
    util::PrefetchPeekable,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Event main type
////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering key is moved out of boxes to allow faster access
pub(super) struct CapSimEvent {
    // Events are ordered by this key. Injector events with equal key need extra comparison, which
    // is done by the injector-specific comparison method
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
    pub(super) fn cmp_injector(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key).then_with(|| match (&self.data, &other.data) {
            (CapSimEventData::InjectorReady(e1), CapSimEventData::InjectorReady(e2)) => e1.cmp_priority(e2),
            _ => Ordering::Equal,
        })
    }
}
impl PartialOrd for CapSimEvent {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CapSimEvent {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
impl PartialEq<Self> for CapSimEvent {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
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
    pub(super) source: CapSimCycleSource,
    pub(super) direction: Direction,
    // Cap change which happens at the end of current cycle, i.e. when this event is processed next.
    // It is kept here instead of the heap, to avoid adding it to the heap and removing it
    pub(super) cycle_end_event: Option<CapSimEvent>,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle source
////////////////////////////////////////////////////////////////////////////////////////////////////
// Periodic sources are infinitely repeating cycles with the same simple output, which is most
// commonly seen in actual fits. All their cycles are the same, so cycle data is calculated just
// once, instead of going through cycle iterator and output instance iterator every cycle
pub(super) enum CapSimCycleSource {
    Periodic(CapSimPeriodic),
    Generic(Box<AggrIter<PValue>>),
}
impl CapSimCycleSource {
    // Sources which never produce any output are not needed for the sim, and None is returned
    pub(super) fn new(iter_data: &AggrIterData<PValue>) -> Option<Self> {
        if let AggrIterData::Regular(inner) = iter_data
            && let CycleSeq::LoopSin(..) = inner.get_cseq()
            && let Some(item) = iter_data.iter().next()
            && let Output::Simple(output) = item.output
        {
            let placement = match item.output_duration_limit {
                Some(limit) if output.delay > limit => return None,
                _ if output.delay == PValue::ZERO => CapSimOutputPlacement::CycleStart,
                _ if output.delay == item.cycle_duration => CapSimOutputPlacement::CycleEnd,
                _ => CapSimOutputPlacement::Delayed(output.delay),
            };
            return Some(Self::Periodic(CapSimPeriodic {
                cycle_duration: item.cycle_duration,
                amount: output.instance,
                placement,
            }));
        }
        Some(Self::Generic(Box::new(iter_data.iter())))
    }
}

#[derive(Copy, Clone)]
pub(super) struct CapSimPeriodic {
    pub(super) cycle_duration: PValue,
    pub(super) amount: PValue,
    pub(super) placement: CapSimOutputPlacement,
}

// Placement of output of a periodic source cycle relatively to the cycle
#[derive(Copy, Clone)]
pub(super) enum CapSimOutputPlacement {
    CycleStart,
    CycleEnd,
    Delayed(PValue),
}
