use std::sync::Arc;

use crate::src::SrcAlias;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Guarded
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub(crate) struct SrcInnerGuarded(Arc<SrcInner>);
impl SrcInnerGuarded {
    pub(crate) fn new(alias: SrcAlias, time_created: time::UtcDateTime, core_src: Arc<rc::Src>) -> Self {
        Self(Arc::new(SrcInner::new(alias, time_created, core_src)))
    }
    pub(crate) fn get_alias(&self) -> SrcAlias {
        self.0.alias
    }
    pub(crate) fn get_time_created(&self) -> time::UtcDateTime {
        self.0.time_created
    }
    pub(crate) fn get_core(&self) -> &Arc<rc::Src> {
        &self.0.core_src
    }
    pub(crate) fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Unguarded
////////////////////////////////////////////////////////////////////////////////////////////////////
struct SrcInner {
    alias: SrcAlias,
    time_created: time::UtcDateTime,
    core_src: Arc<rc::Src>,
}
impl SrcInner {
    fn new(alias: SrcAlias, time_created: time::UtcDateTime, core_src: Arc<rc::Src>) -> Self {
        Self {
            alias,
            time_created,
            core_src,
        }
    }
}
