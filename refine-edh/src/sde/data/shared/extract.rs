pub(in crate::sde) trait ExtractOne<EVE> {
    fn extract(self, extracted: &mut Vec<EVE>);
}

pub(in crate::sde) trait ExtractTwo<EVE1, EVE2> {
    fn extract(self, extracted1: &mut Vec<EVE1>, extracted2: &mut Vec<EVE2>);
}
