use crate::nd::{NEffectProjGetter, NEffectResist};

pub(crate) struct NEffectProjModSpec
{
    pub(crate) proj_mult: Option<NEffectProjGetter> = None,
    pub(crate) resist: NEffectResist = NEffectResist::Standard,
}
