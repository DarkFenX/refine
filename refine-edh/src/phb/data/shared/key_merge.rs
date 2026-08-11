pub(in crate::phb) type Key = i32;

pub(in crate::phb) trait KeyMergeOne<EVE> {
    fn key_merge(self, key: Key) -> Vec<EVE>;
}

pub(in crate::phb) trait KeyMergeTwo<EVE1, EVE2> {
    fn key_merge(self, key: Key) -> (Vec<EVE1>, Vec<EVE2>);
}
