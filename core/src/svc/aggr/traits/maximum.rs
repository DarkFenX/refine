pub(in crate::svc::aggr) trait Maximum {
    fn maximum(self, other: Self) -> Self;
}
