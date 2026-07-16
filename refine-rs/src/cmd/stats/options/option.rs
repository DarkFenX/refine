pub enum StatOption<T>
where
    T: Clone + Default,
{
    Default,
    Disabled,
    Enabled,
    EnabledOptions(Vec<T>),
}
