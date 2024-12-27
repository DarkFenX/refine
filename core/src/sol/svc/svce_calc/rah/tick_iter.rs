use crate::{
    defs::{AttrVal, SolItemId},
    util::StMap,
};

use super::rah_info::SolRahInfo;

pub(super) struct SolRahSimTickData {
    time: AttrVal,
    cycled: Vec<SolItemId>,
    cycling_data: StMap<SolItemId, AttrVal>,
}

pub(super) struct SolRahSimTickIter<'a> {
    info: &'a StMap<SolItemId, SolRahInfo>,
}
impl<'a> SolRahSimTickIter<'a> {
    pub(super) fn new(info: &'a StMap<SolItemId, SolRahInfo>) -> Self {
        Self { info }
    }
}
impl<'a> Iterator for SolRahSimTickIter<'a> {
    type Item = SolRahSimTickData;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
