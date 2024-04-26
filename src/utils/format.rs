use regex::Regex;

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
    date.format(format.unwrap_or("%Y-%m-%d %H:%M")).to_string()
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
    // replace url to link or image
    let replaced_text = Regex::new(r"(?P<url>https?://\S+)").unwrap().replace_all(
        content,
        |caps: &regex::Captures| {
            let url = &caps[1];
            // 如果是图片
            let url_upper = url.to_uppercase();
            if url_upper.ends_with(".JPG")
                || url_upper.ends_with(".PNG")
                || url_upper.ends_with(".JPEG")
                || url_upper.ends_with(".GIF")
                || url_upper.ends_with(".BMP")
                || url_upper.ends_with(".WEBP")
                || url_upper.ends_with(".SVG")
                || url_upper.ends_with(".ICO")
                || url_upper.ends_with(".AVIF")
                || url_upper.ends_with(".APNG")
            {
                format!(r#"<img class="post-image" src="{}" alt="Image">"#, url)
            } else {
                format!(
                    r#"<a class="post-link" href="{}" target="_blank">{}</a>"#,
                    url, url
                )
            }
        },
    );

    // replace '#xxx' to '<a class="post-tag-link" href="javascript:void(0)" target="_blank">#xxx</a>'
    let replaced_text = Regex::new(r"#\S+(?: |$)").unwrap().replace_all(
        &replaced_text,
        |caps: &regex::Captures| {
            let tag = caps.get(0).unwrap().as_str();
            format!(
                r#"<a class="post-tag-link" href="javascript:void(0)" target="_blank">{}</a>"#,
                tag
            )
        },
    );

    // replace '\n' to '<br>'
    let replaced_text = Regex::new(r"\\n").unwrap().replace_all(&replaced_text, "<br>");

    replaced_text.to_string()
}
