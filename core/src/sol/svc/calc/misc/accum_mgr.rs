use crate::sol::svc::calc::ModAccumFast;

pub(in crate::sol::svc::calc) struct AccumMgr {
    accums: Vec<ModAccumFast>,
}
impl AccumMgr {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self { accums: Vec::new() }
    }
    pub(in crate::sol::svc::calc) fn allocate_accum(&mut self) -> ModAccumFast {
        if self.accums.is_empty() {
            return ModAccumFast::new();
        }
        let mut accum = self.accums.swap_remove(0);
        accum.clear();
        accum
    }
    pub(in crate::sol::svc::calc) fn return_accum(&mut self, accum: ModAccumFast) {
        self.accums.push(accum);
    }
}
