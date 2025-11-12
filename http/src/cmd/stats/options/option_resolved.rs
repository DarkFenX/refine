use crate::cmd::stats::options::HStatOption;

pub(in crate::cmd::stats) struct HStatResolvedOption<T> {
    pub(in crate::cmd::stats) enabled: bool,
    pub(in crate::cmd::stats) options: Vec<T>,
}
impl<T> HStatResolvedOption<T>
where
    T: Clone + Default,
{
    pub(in crate::cmd::stats) fn new(root_opt: &Option<HStatOption<T>>, default: bool) -> Self {
        match root_opt {
            Some(inner_opt) => HStatResolvedOption {
                enabled: inner_opt.is_enabled(),
                options: inner_opt.get_extended_options(),
            },
            None => match default {
                true => HStatResolvedOption {
                    enabled: true,
                    options: vec![T::default()],
                },
                // No need to allocate anything if check is disabled
                false => HStatResolvedOption {
                    enabled: false,
                    options: Vec::new(),
                },
            },
        }
    }
}
