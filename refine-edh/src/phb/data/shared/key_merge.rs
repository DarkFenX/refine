pub(in crate::phb) type Key = i32;

pub(in crate::phb) trait KeyMergeOne<EVE> {
    fn key_merge(self, key: Key, merged: &mut Vec<EVE>);
}

pub(in crate::phb) trait KeyMergeTwo<EVE1, EVE2> {
    fn key_merge(self, key: Key, merged1: &mut Vec<EVE1>, merged2: &mut Vec<EVE2>);
}
