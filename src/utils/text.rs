pub fn parse_command(text: &str) -> (&str, &str) {
    match text.find(' ') {
        Some(pos) => (&text[..pos], &text[pos + 1..]),
        None => (text, ""),
    }
}
