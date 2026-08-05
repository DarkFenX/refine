use crate::nd::{NEffectProjMultGetter, NEffectResist};

pub(crate) struct NEffectProjModSpec
{
    pub(crate) proj_mult: Option<NEffectProjMultGetter> = None,
    pub(crate) resist: NEffectResist = NEffectResist::Standard,
}
