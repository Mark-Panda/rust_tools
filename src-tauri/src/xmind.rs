//! XMind (.xmind) parsing and conversion to Markdown.
//! .xmind is a ZIP containing content.json (and optionally content.xml for older versions).
//! We only support content.json (newer XMind format).

use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::ZipArchive;

#[derive(Deserialize)]
struct RootTopic {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    notes: Option<NotesContent>,
    #[serde(default)]
    children: Option<ChildrenWrapper>,
}

#[derive(Deserialize, Default)]
struct NotesContent {
    #[serde(default)]
    plain: Option<PlainContent>,
}

#[derive(Deserialize, Default)]
struct PlainContent {
    #[serde(default)]
    content: Option<String>,
}

/// Children: { "attached": [ Topic ] } in XMind content.json
#[derive(Deserialize, Default)]
struct ChildrenWrapper {
    #[serde(default)]
    attached: Vec<Topic>,
}

#[derive(Deserialize, Default)]
struct Topic {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    notes: Option<NotesContent>,
    #[serde(default)]
    children: Option<ChildrenWrapper>,
}

/// content.json can be: [ { rootTopic: {...} }, ... ] or direct { rootTopic }.
fn parse_content_json(json_str: &str) -> Result<RootTopic, String> {
    let v: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| format!("content.json 解析失败: {}", e))?;

    let root = if let Some(arr) = v.as_array() {
        let first = arr.first().ok_or("content.json 为空数组")?;
        first
            .get("rootTopic")
            .ok_or("content.json 中缺少 rootTopic")?
            .clone()
    } else if let Some(root_val) = v.get("rootTopic") {
        root_val.clone()
    } else {
        return Err("content.json 中缺少 rootTopic".to_string());
    };

    let root: RootTopic = serde_json::from_value(root).map_err(|e| format!("rootTopic 结构错误: {}", e))?;
    Ok(root)
}

fn extract_plain_notes(notes: &Option<NotesContent>) -> Option<String> {
    let n = notes.as_ref()?;
    let plain = n.plain.as_ref()?;
    let content = plain.content.as_ref()?;
    let s = content.trim();
    if s.is_empty() {
        None
    } else {
        Some(strip_html(s))
    }
}

fn strip_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .trim()
        .to_string()
}

fn topic_title(t: &Topic) -> &str {
    t.title.as_deref().unwrap_or("")
}

fn root_title(r: &RootTopic) -> &str {
    r.title.as_deref().unwrap_or("")
}

fn write_markdown_from_root(root: &RootTopic, out: &mut String) {
    let title = root_title(root);
    out.push_str("# ");
    out.push_str(&escape_md_line(title));
    out.push('\n');
    if let Some(notes) = extract_plain_notes(&root.notes) {
        out.push_str("\n> ");
        out.push_str(&escape_blockquote(&notes));
        out.push_str("\n\n");
    }
    if let Some(ref children) = root.children {
        for topic in &children.attached {
            write_topic(topic, 2, out);
        }
    }
}

fn write_topic(topic: &Topic, level: u32, out: &mut String) {
    let title = topic_title(topic);
    if level <= 6 {
        let hashes = "#".repeat(level as usize);
        out.push_str("\n");
        out.push_str(&hashes);
        out.push(' ');
        out.push_str(&escape_md_line(title));
        out.push('\n');
    } else {
        let indent = "  ".repeat((level - 7) as usize);
        out.push_str("\n");
        out.push_str(&indent);
        out.push_str("- ");
        out.push_str(&escape_md_line(title));
        out.push('\n');
    }
    if let Some(notes) = extract_plain_notes(&topic.notes) {
        out.push_str("\n> ");
        out.push_str(&escape_blockquote(&notes));
        out.push_str("\n\n");
    }
    if let Some(ref children) = topic.children {
        for child in &children.attached {
            write_topic(child, level + 1, out);
        }
    }
}

fn escape_md_line(s: &str) -> String {
    s.replace('\n', " ")
}

fn escape_blockquote(s: &str) -> String {
    s.replace("\n", "\n> ")
}

/// Normalize zip entry name: use forward slash, no leading slash.
fn normalize_zip_name(n: &str) -> String {
    n.replace('\\', "/").trim_matches('/').to_string()
}

/// Find content.json in zip: root "content.json" or any path ending with "/content.json".
fn find_content_json(names: &[String]) -> Option<&str> {
    // Prefer exact root "content.json"
    if names.iter().any(|n| normalize_zip_name(n) == "content.json") {
        return names.iter().find(|n| normalize_zip_name(n) == "content.json").map(String::as_str);
    }
    // Then any path whose last segment is content.json (e.g. contents/content.json)
    names
        .iter()
        .find(|n| {
            let norm = normalize_zip_name(n);
            norm == "content.json" || norm.ends_with("/content.json")
        })
        .map(String::as_str)
}

/// Read .xmind (ZIP), find content.json, parse and convert to Markdown string (UTF-8).
pub fn parse_and_convert(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    if path.extension().map(|e| e != "xmind").unwrap_or(true) {
        return Err("请选择 .xmind 文件".to_string());
    }
    let file = File::open(path).map_err(|e| format!("无法打开文件: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("不是有效的 ZIP/xmind 文件: {}", e))?;

    let names: Vec<String> = archive.file_names().map(String::from).collect();
    let json_name = find_content_json(&names);

    if json_name.is_none() {
        let has_xml = names
            .iter()
            .any(|n| normalize_zip_name(n) == "content.xml" || normalize_zip_name(n).ends_with("/content.xml"));
        if has_xml {
            return Err(
                "该文件为 XMind 8 旧版格式（content.xml），当前仅支持 XMind Zen/新版（content.json）。请在 XMind 中「另存为」或「导出」为新版 .xmind 后再试。".to_string(),
            );
        }
        let list: String = names.iter().take(20).map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        let more = if names.len() > 20 { " ..." } else { "" };
        return Err(format!(
            "未找到 content.json，可能是不支持的 XMind 版本。ZIP 内文件: {}{}",
            list, more
        ));
    }

    let name = json_name.unwrap();
    let content = {
        let mut entry = archive
            .by_name(name)
            .map_err(|e| format!("读取 content.json 失败: {}", e))?;
        let mut s = String::new();
        entry
            .read_to_string(&mut s)
            .map_err(|e| format!("读取 content.json 失败: {}", e))?;
        s
    };

    let root = parse_content_json(&content)?;
    let mut md = String::new();
    write_markdown_from_root(&root, &mut md);
    Ok(md)
}

/// Write Markdown string to a file (UTF-8).
pub fn save_markdown(path: &str, content: &str) -> Result<(), String> {
    let path = Path::new(path);
    let mut f = File::create(path).map_err(|e| format!("无法创建文件: {}", e))?;
    f.write_all(content.as_bytes())
        .map_err(|e| format!("写入失败: {}", e))?;
    Ok(())
}
