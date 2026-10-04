// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Helpers for the opt-in inline code language prefix extension.

/// Find a valid language prefix ending immediately before an inline code span.
///
/// The return value contains the byte offset of the prefix and its ASCII
/// language token. The accepted characters follow `CMarkly`'s extension.
pub(crate) fn before(bytes: &[u8], code_start: usize) -> Option<(usize, &str)> {
    if code_start == 0 || bytes.get(code_start - 1) != Some(&b':') {
        return None;
    }

    let end = code_start - 1;
    let mut start = end;

    while start > 0 && is_info_char(bytes[start - 1]) {
        start -= 1;
    }

    if start == end || !bytes.get(start).copied().is_some_and(is_info_start_char) {
        return None;
    }

    let info = core::str::from_utf8(&bytes[start..end])
        .expect("inline code info contains only accepted ASCII characters");
    Some((start, info))
}

fn is_info_start_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
}

fn is_info_char(byte: u8) -> bool {
    is_info_start_char(byte) || matches!(byte, b'_' | b'-' | b'+' | b'#' | b'.')
}

#[cfg(test)]
mod tests {
    use super::before;

    #[test]
    fn recognizes_only_prefixed_info_tokens() {
        assert_eq!(before(b"rust:`code`", 5), Some((0, "rust")));
        assert_eq!(before(b"-:`code`", 2), None);
        assert_eq!(before(b"code", 4), None);
        assert_eq!(before(b":code", 0), None);
    }
}
