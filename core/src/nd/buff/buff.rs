use crate::ad::{ABuff, ABuffId};

pub(crate) type NBuffMaker = fn() -> ABuff;

pub(crate) struct NBuff {
    // Adapted data buff ID
    pub(crate) aid: ABuffId,
    // Fields related to adapted data generation
    pub(crate) adg_make_buff_fn: Option<NBuffMaker> = None,
}
