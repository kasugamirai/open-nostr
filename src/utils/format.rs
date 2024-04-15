/// format public key
/// 
/// # Parameters
/// 
/// * `public_key`: public key
/// * `len`: length
/// 
/// # Returns
/// 
/// formatted public key
/// 
/// # Examples
/// 
/// ```
/// use crate::utils::format::format_public_key;
/// 
/// let public_key = "5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3";
/// let formatted = format_public_key(public_key, None);
/// assert_eq!(formatted, "5KQwrP");
pub fn format_public_key(public_key: &str, len: Option<usize>) -> String {
    let mut public_key = public_key.to_string();
    let len = len.unwrap_or(6);
    public_key.truncate(len);
    public_key
}

/// format timestamp
///
/// # Parameters
///
/// * `timestamp`: timestamp
/// * `format`: format string
///
/// # Returns
///
/// formatted timestamp
///
/// # Examples
///
/// ```
/// use crate::utils::format::format_timestamp;
///
/// let timestamp = chrono::Utc::ymd(2022, 1, 1).and_hms(0, 1, 0).timestamp();
///
/// let formatted = format_timestamp(timestamp, Some("%Y-%m-%d %H:%M:%S"));
///
/// assert_eq!(formatted, "2022-01-01 00:01:00");
pub fn format_timestamp(timestamp: u64, format: Option<&str>) -> String {
    let date = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap();
    date.format(format.unwrap_or("%Y-%m-%d %H:%M:%S"))
        .to_string()
}

/// format create time
///
/// # Examples
///
/// ```
/// use crate::utils::format::format_create_at;
///
/// let timestamp = chrono::Utc::now().timestamp() - 60;
/// let formatted = format_create_at(timestamp);
/// assert_eq!(formatted, "1 minutes ago");
/// ```
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

/// format post content
///
/// # Examples
///
/// ```
/// use crate::utils::format::format_content;
///
/// let content = "https://www.google.com";
/// let formatted_content = format_content(content);
/// assert_eq!(formatted_content, "<a class=\"post-link\" href=\"https://www.google.com\" target=\"_blank\">https://www.google.com</a>");
/// ```
pub fn format_content(content: &str) -> String {
    // replace http or https link to 'a' tag use regex
    let content = regex::Regex::new(r"(?P<url>https?://\S+)")
        .unwrap()
        .replace_all(
            content,
            "<a class=\"post-link\" href=\"$url\" target=\"_blank\">$url</a>",
        );

    content.to_string()
}
