pub use rc::src::SrcWarnings;

pub use crate::{
    api::{Src, SrcAlias},
    info::{SrcInfo, SrcInfoExt, SrcInfoMode, SrcOrigin, SrcOriginGeneratedReason},
};

pub mod err {
    pub use crate::api::{AddSrcError, GetSrcError, RemoveSrcError, SrcAliasPruneInitError, SrcAliasStrictInitError};
}
