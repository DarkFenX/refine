pub use rc::src::{SrcOrigin, SrcOriginGeneratedReason, SrcWarnings};

pub use crate::{
    api::{Src, SrcAlias},
    info::{
        SrcInfo, SrcInfoFull, SrcInfoMode, SrcInfoPartial, SrcItemInfo, SrcItemInfos, SrcMutatedItemBaseInfo,
        SrcMutatedItemInfo, SrcMutatedItemInfos,
    },
};

pub mod err {
    pub use rc::src::err::SrcInitError;

    pub use crate::api::{SrcAddError, SrcAliasPruneInitError, SrcAliasStrictInitError, SrcGetError, SrcRemoveError};
}
