use crate::{
    ad::{ABuff, ABuffId},
    ed::EBuffId,
};

pub(crate) type NBuffMaker = fn() -> ABuff;

pub(crate) struct NBuff {
    // EVE data buff ID
    pub(crate) eid: Option<EBuffId>,
    // Adapted data buff ID
    pub(crate) aid: ABuffId,
    // Fields related to adapted data generation
    pub(crate) adg_make_buff_fn: Option<NBuffMaker> = None,
}
