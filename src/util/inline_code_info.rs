//! Helpers for the opt-in inline code language prefix extension.

/// Find a valid language prefix ending immediately before an inline code span.
///
/// The return value contains the byte offset of the prefix and its ASCII
/// language token. The accepted characters follow CMarkly's extension.
pub(crate) fn before(bytes: &[u8], code_start: usize) -> Option<(usize, &str)> {
    if code_start == 0 || bytes.get(code_start - 1) != Some(&b':') {
        return None;
    }

    let end = code_start - 1;
    let mut start = end;

    while start > 0 && is_info_char(bytes[start - 1]) {
        start -= 1;
    }

    if start == end || !bytes.get(start).copied().map_or(false, is_info_start_char) {
        return None;
    }

    if start > 0 && is_info_char(bytes[start - 1]) {
        return None;
    }

    let info = core::str::from_utf8(&bytes[start..end]).ok()?;
    Some((start, info))
}

fn is_info_start_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
}

fn is_info_char(byte: u8) -> bool {
    is_info_start_char(byte) || matches!(byte, b'_' | b'-' | b'+' | b'#' | b'.')
}
