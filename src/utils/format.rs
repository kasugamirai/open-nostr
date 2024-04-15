pub fn format_public_key(public_key: &str) -> String {
    let mut public_key = public_key.to_string();
    public_key.truncate(4);
    public_key
}

pub fn format_timestamp(timestamp: u64, format: Option<&str>) -> String {
    let date = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap();
    date.format(format.unwrap_or("%Y-%m-%d %H:%M:%S"))
        .to_string()
}

pub fn format_create_at(timestamp: u64) -> String {
    let current = chrono::Utc::now().timestamp();
    let ts = timestamp as i64;
    // format to {} minutes age / {} hours age / {} days ago
    if current - ts < 60 {
        format!("{} seconds ago", current - ts)
    } else if current - ts < 60 * 60 {
        format!("{} minutes ago", (current - ts) / 60)
    } else if current - ts < 60 * 60 * 24 {
        format!("{} hours ago", (current - ts) / 60 / 60)
    } else if current - ts < 60 * 60 * 24 * 30 {
        format!("{} days ago", (current - ts) / 60 / 60 / 24)
    } else {
        format_timestamp(timestamp, None)
    }
}

pub fn format_content(content: &str) -> String {
    // replace http or https link to 'a' tag use regex
    let content = regex::Regex::new(r"(?P<url>https?://\S+)").unwrap().replace_all(content, "<a class=\"post-link\" href=\"$url\" target=\"_blank\">$url</a>");

    content.to_string()
}
