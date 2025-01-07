use crate::{sol::svc::SolSvc, src::Src};

impl SolSvc {
    pub(in crate::sol) fn src_changed(&mut self, src: &Src) {
        self.notify_src_changed(src)
    }
}
