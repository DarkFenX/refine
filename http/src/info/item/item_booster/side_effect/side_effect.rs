use super::HSideEffectStr;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HSideEffectInfo {
    chance: rc::AttrVal,
    state: bool,
    strength: Option<HSideEffectStr>,
}
impl From<rc::FullSideEffectMut<'_>> for HSideEffectInfo {
    fn from(mut core_side_effect: rc::FullSideEffectMut) -> Self {
        Self {
            chance: core_side_effect.get_chance(),
            state: core_side_effect.get_state(),
            strength: core_side_effect
                .get_strength()
                .and_then(|strength| strength.try_into().ok()),
        }
    }
}
