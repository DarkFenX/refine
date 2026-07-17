pub use crate::{
    api::{EdSource, Src, SrcAlias},
    info::{SrcInfo, SrcInfoExt, SrcInfoMode, SrcOrigin, SrcOriginGeneratedReason, SrcWarnings},
};

pub mod err {
    pub use crate::api::{AddSrcError, GetSrcError, RemoveSrcError};
}
