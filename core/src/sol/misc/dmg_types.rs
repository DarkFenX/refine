#[derive(Copy, Clone)]
pub struct SolDmgTypes<T> {
    em: T,
    thermal: T,
    kinetic: T,
    explosive: T,
}
impl<T> SolDmgTypes<T> {
    pub(crate) fn new(em: T, thermal: T, kinetic: T, explosive: T) -> Self {
        Self {
            em,
            thermal,
            kinetic,
            explosive,
        }
    }
}
