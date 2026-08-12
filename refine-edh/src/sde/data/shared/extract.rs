pub(in crate::sde) trait ExtractOne<EVE> {
    fn extract(self) -> Vec<EVE>;
}

pub(in crate::sde) trait ExtractTwo<EVE1, EVE2> {
    fn extract(self) -> (Vec<EVE1>, Vec<EVE2>);
}
