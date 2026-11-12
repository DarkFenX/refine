const MAX_PREALLOC_BYTES: usize = 8 * 1024 * 1024;

pub(in crate::ad::data) fn get_map_prealloc_capacity<K, V>(size_hint: Option<usize>) -> usize {
    // Protection against wrong size hints in binary data
    size_hint
        .unwrap_or(0)
        .min(MAX_PREALLOC_BYTES / size_of::<(K, V)>().max(1))
}
