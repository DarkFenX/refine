use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct HSrc {
    inner: Arc<rc::Src>,
}
impl HSrc {
    pub(super) fn get_core(&self) -> &rc::Src {
        &self.inner
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSrc {
    pub(super) fn from_core(core_src: rc::Src) -> Self {
        Self {
            inner: Arc::new(core_src),
        }
    }
}
