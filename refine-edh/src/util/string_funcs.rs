pub(crate) fn cap_len(text: String, len_limit: usize) -> String {
    let text_len = text.chars().count();
    if text_len <= len_limit {
        return text;
    }
    let kept: String = text.chars().take(len_limit).collect();
    format!("{kept}<{} more characters hidden>", text_len - len_limit)
}
