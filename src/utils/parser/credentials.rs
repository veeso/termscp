pub(super) fn optional_capture(groups: &regex::Captures<'_>, index: usize) -> Option<String> {
    groups.get(index).map(|group| group.as_str().to_string())
}

pub(super) fn required_capture(
    groups: &regex::Captures<'_>,
    index: usize,
    field_name: &str,
) -> Result<String, String> {
    optional_capture(groups, index).ok_or_else(|| format!("Missing {field_name}"))
}
