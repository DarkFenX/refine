use crate::{
    defs::{AttrVal, SolItemId},
    util::StMap,
};

use super::info::SolRahInfo;

pub(super) struct SolRahSimTickData {
    time: AttrVal,
    cycled: Vec<SolItemId>,
    cycling_data: StMap<SolItemId, AttrVal>,
}

pub(super) struct SolRahSimTickIterRahData {
    info: SolRahInfo,
    cycling_time: AttrVal,
}
impl SolRahSimTickIterRahData {
    fn new(info: SolRahInfo) -> Self {
        Self {
            info,
            cycling_time: 0.0,
        }
    }
}

pub(super) struct SolRahSimTickIter {
    tick: usize,
    rah_data: StMap<SolItemId, SolRahSimTickIterRahData>,
}
impl SolRahSimTickIter {
    pub(super) fn new(infos: &StMap<SolItemId, SolRahInfo>) -> Self {
        let mut rah_data = StMap::with_capacity(infos.len());
        for (item_id, info) in infos.iter() {
            rah_data.insert(*item_id, SolRahSimTickIterRahData::new(*info));
        }
        Self { tick: 0, rah_data }
    }
}
impl Iterator for SolRahSimTickIter {
    type Item = SolRahSimTickData;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.tick >= 500 {
                return None;
            }
            self.tick += 1;
            // Pick time remaining until some RAH finishes its cycle
            let time_passed = self
                .rah_data
                .values()
                .map(|v| v.info.cycle_time - v.cycling_time)
                .min_by(|a, b| a.total_cmp(b))
                .unwrap();
            // let mut cycled = Vec::new();
        }
    }
}
