use std::sync::Arc;

#[derive(Clone)]
pub struct Src {
    inner: Arc<rc::Src>,
}
impl Src {
    pub fn get_core(&self) -> &rc::Src {
        &self.inner
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src {
    pub(super) fn from_core(core_src: rc::Src) -> Self {
        Self {
            inner: Arc::new(core_src),
        }
    }
}
