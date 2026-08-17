pub use rc::src::{SrcOrigin, SrcOriginGeneratedReason, SrcWarnings};

pub use crate::{
    api::{Src, SrcAlias, SrcInfoArgs},
    info::{SrcInfo, SrcInfoExt, SrcInfoMode},
};

pub mod err {
    pub use crate::api::{SrcAddError, SrcAliasPruneInitError, SrcAliasStrictInitError, SrcGetError, SrcRemoveError};
}
