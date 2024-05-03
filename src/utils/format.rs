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
    let replaced_text = replace_urls(content);
    let replaced_text = replace_tags(&replaced_text);
    replace_newlines(&replaced_text)
}

fn replace_urls(content: &str) -> String {
    let re = Regex::new(r"(?P<url>https?://\S+)").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let url = &caps[1];
        let url_upper = url.to_uppercase();
        if is_image(&url_upper) {
            format!(
                r#"<img class="post-image media" src="{}" alt="Image">"#,
                url
            )
        } else if is_video(&url_upper) {
            format!(
                r#"<video class="post-video media" src="{}" controls></video>"#,
                url
            )
        } else {
            format!(
                r#"<a class="post-link" href="{}" target="_blank">{}</a>"#,
                url, url
            )
        }
    })
    .to_string()
}

fn is_image(url: &str) -> bool {
    let extensions = [
        ".JPG", ".PNG", ".JPEG", ".GIF", ".BMP", ".WEBP", ".SVG", ".ICO", ".AVIF", ".APNG",
    ];
    extensions.iter().any(|ext| url.ends_with(ext))
}

fn is_video(url: &str) -> bool {
    let extensions = [
        ".MOV", ".MP4", ".MKV", ".AVI", ".WEBM", ".WMV", ".MPG", ".MPEG", ".FLV", ".F4V", ".M4V",
    ];
    extensions.iter().any(|ext| url.ends_with(ext))
}

fn replace_tags(content: &str) -> String {
    let re = Regex::new(r"#\S+(?: |$)").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let tag = caps.get(0).unwrap().as_str();
        format!(
            r#"<a class="post-tag-link" href="javascript:void(0)" target="_blank">{}</a>"#,
            tag
        )
    })
    .to_string()
}

fn replace_newlines(content: &str) -> String {
    let re = Regex::new(r"\\n").unwrap();
    re.replace_all(content, "<br>").to_string()
}
