use crate::{
    src::Src,
    ss::{fit::SsFits, item::SsItems},
};

pub(in crate::ss) struct SsView<'a> {
    pub(in crate::ss) src: &'a Src,
    pub(in crate::ss) fits: &'a SsFits,
    pub(in crate::ss) items: &'a SsItems,
}
impl<'a> SsView<'a> {
    pub(in crate::ss) fn new(src: &'a Src, fits: &'a SsFits, items: &'a SsItems) -> Self {
        Self { src, fits, items }
    }
}
