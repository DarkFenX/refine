use crate::{sol::svc::SolSvcs, src::Src};

impl SolSvcs {
    pub(in crate::sol) fn src_changed(&mut self, src: &Src) {
        self.notify_src_changed(src)
    }
}
