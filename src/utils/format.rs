use dioxus::prelude::*;
use nostr_sdk::{EventId, FromBech32, PublicKey};
use regex::Regex;
use nostr_sdk::nips::nip19::Nip19Event;
use crate::components::{Mention, Quote};
use crate::nostr::utils::{is_note_address, AddressType};
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

///
/// format note content
pub fn format_note_content(data: &str, relay_name: &str) -> Option<VNode> {
    let mut element: Option<VNode> = rsx! {
        div {
            class: "text pl-52",
            "Note Loading..."
        }
    };
    let mut elements = vec![];

    let mut parts = Vec::new();
    let mut last_end = 0;

    let re: Regex = Regex::new(r"(nostr:[a-zA-Z0-9]{63})").unwrap();
    for mat in re.find_iter(data) {
        if mat.start() > last_end {
            parts.push(&data[last_end..mat.start()]);
        }
        parts.push(mat.as_str());
        last_end = mat.end();
    }

    if last_end < data.len() {
        parts.push(&data[last_end..]);
    }
    for i in parts {
        if i.starts_with("nostr:") {
            let id = i.strip_prefix("nostr:").unwrap();
            let is_note = is_note_address(i);
            let _el = if is_note == AddressType::Note {
                rsx! {
                    Quote {
                        event_id: EventId::from_bech32(id).unwrap(),
                        relay_name: relay_name,
                        quote_nostr: i.to_string(),
                    }
                }
            } else if is_note == AddressType::Mention {
                rsx! {
                    Mention {
                        pubkey: PublicKey::from_bech32(id).unwrap(),
                        relay_name: relay_name.to_string(),
                    }
                }
            } else {
                rsx! { span {
                    "{i}"
                } }
            };
            elements.push(rsx! {
                {_el}
            })
        } else {
            elements.push(rsx! {
                span {
                    dangerous_inner_html: "{format_content(i)}",
                }
            });
        }
    }

    element = rsx! {
        for element in elements {
            {element}
        }
    };
    rsx! {
        div {
            class: "text pl-52",
            {element}
        }
    }
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
        let t = (current - ts) / 60 / 60;
        if t == 1 {
            format!("{} hour ago", t)
        } else {
            format!("{} hours ago", t)
        }
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
    let replaced_text = add_media_wrapper(&replaced_text);
    let replaced_text = replace_qoutes(&replaced_text);
    replace_newlines(&replaced_text)
}

pub fn splite_by_replys(content: &str) -> Vec<String> {
    let content = &format_content(content);
    let re = Regex::new(r"(nostr:note[a-zA-Z0-9]{59})").unwrap();

    let mut parts = Vec::new();
    let mut last_end = 0;

    for mat in re.find_iter(content) {
        if mat.start() > last_end {
            parts.push(&content[last_end..mat.start()]);
        }
        parts.push(mat.as_str());
        last_end = mat.end();
    }

    if last_end < content.len() {
        parts.push(&content[last_end..]);
    }
    parts.iter().map(|part| part.to_string()).collect()
}

fn replace_urls(content: &str) -> String {
    let re = Regex::new(r"(?P<url>https?://\S+)").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let url = &caps[1];
        let url_upper = url.to_uppercase();
        if is_image(&url_upper) {
            format!(
                r#"<img class="post-image media" src="{}" alt="Image" data-type="media" />"#,
                url
            )
        } else if is_video(&url_upper) {
            format!(
                r#"<video class="post-video media" src="{}" controls data-type="media" />"#,
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

fn add_media_wrapper(content: &str) -> String {
    let mut content = String::from(content);
    if let Some(index) = content.find("<img") {
        content.insert_str(index, "<div class=\"post-media-wrap\">");
    } else if let Some(index) = content.find("<video") {
        content.insert_str(index, "<div class=\"post-media-wrap\">");
    }
    if let Some(index) = content.rfind("data-type=\"media\" />") {
        let index = index + "data-type=\"media\" />".len();
        content.insert_str(index, "</div>");
    }
    content
}

// nostr:note1kwqrjx93xex7rdpqhc6d2ltexrmvt6jm7t7wufq9qvqhka64um0s3yyuxd
fn replace_qoutes(content: &str) -> String {
    // 为什么结果是空的呢？
    let re = Regex::new(r"nostr:note[a-zA-Z0-9]{59}").unwrap();
    re.replace_all(content, |caps: &regex::Captures| {
        let note_id = &caps[0];
        log::info!("note_id: {}", note_id);
        format!(
            r#"<a class="post-link" href="javascript:void(0)">{}</a>"#,
            note_id
        )
    })
    .to_string()
}


pub fn parse_notif_content_event(content: &str) -> Option<Nip19Event> {
    let re: Regex = Regex::new(r"nostr:nevent1\w+").unwrap();
    if let Some(capture) = re.captures(&content) {
        if let Some(nevent) = capture.get(0) {
            let event_id_str: &str = nevent.as_str();
            let nevent_str: &str = event_id_str.strip_prefix("nostr:").unwrap();
            match Nip19Event::from_bech32(nevent_str){
                Ok(nev) => {
                    return Some(nev);
                }
                Err(e) => {
                    tracing::info!("Failed to decode {}: {:?}", nevent_str, e);
                }
            }
        }
    }
    return None;
}

pub fn remove_content_nostr_str(content: &str) -> String {
    let re: Regex = Regex::new(r"nostr:[^\s]+").unwrap();
    return re.replace_all(content, "").to_string();
}
