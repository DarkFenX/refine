use crate::nd::{NEffectProjGetter, NEffectResist};

pub(crate) struct NEffectProjModSpec
{
    pub(crate) proj_mult: NEffectProjGetter,
    pub(crate) resist: NEffectResist = NEffectResist::Standard,
}
