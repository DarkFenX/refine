pub use rc::src::{SrcOriginGeneratedReason, SrcWarnings};

pub use crate::{
    api::{Src, SrcAlias},
    info::{SrcInfo, SrcInfoExt, SrcInfoMode, SrcOrigin},
};

pub mod err {
    pub use crate::api::{AddSrcError, GetSrcError, RemoveSrcError, SrcAliasPruneInitError, SrcAliasStrictInitError};
}
