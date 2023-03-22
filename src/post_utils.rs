#[cfg(engine)]
use pulldown_cmark::{html, Options, Parser};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
#[cfg(engine)]
use std::fs;
#[cfg(engine)]
use std::path::Path;

/// Parses the given post into a minimal representation.
#[cfg(engine)]
pub fn parse_slim_post(path: &Path) -> Result<SlimPost, anyhow::Error> {
    let file_contents = fs::read_to_string(path)?;
    let (frontmatter, _) = parse_frontmatter(file_contents, false);

    let date = frontmatter.get("date").expect("no date in post").to_string();

    // Create a new slim representation of the post with the parsed frontmatter
    let slim_post = SlimPost {
        title: frontmatter.get("title").expect("no title in post").to_string(),
        date: parse_date(date),
        description: frontmatter.get("description").expect("no description in post").to_string(),
        slug: path.file_name().unwrap().to_string_lossy().to_string().strip_suffix(".md").unwrap().to_string(),
    };

    Ok(slim_post)
}

/// Parses the given post into a full representation.
#[cfg(engine)]
pub fn parse_full_post(path: &Path) -> Result<Post, anyhow::Error> {
    let file_contents = fs::read_to_string(path)?;
    let (frontmatter, contents) = parse_frontmatter(file_contents, true);

    let date = frontmatter.get("date").expect("no date in post").to_string();

    // Create a new full representation of the post with the parsed frontmatter
    let slim_post = Post {
        title: frontmatter.get("title").expect("no title in post").to_string(),
        date: parse_date(date),
        description: frontmatter.get("description").expect("no description in post").to_string(),
        slug: path.file_name().unwrap().to_string_lossy().to_string().strip_suffix(".md").unwrap().to_string(),
        contents,
    };

    Ok(slim_post)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    /// The title of the post.
    pub title: String,
    /// A short description of the post.
    pub description: String,
    /// The date: year, month, and day.
    pub date: (u32, u8, u8),
    /// The post's filename slug.
    pub slug: String,
    /// The HTML contents of the post.
    pub contents: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SlimPost {
    /// The title of the post.
    pub title: String,
    /// A short description of the post.
    pub description: String,
    /// The date: year, month, and day.
    pub date: (u32, u8, u8),
    /// The post's filename slug.
    pub slug: String,
}

#[cfg(client)]
#[wasm_bindgen::prelude::wasm_bindgen(module = "/src/date.js")]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_name = "formatDate")]
    pub fn format_date(year: u32, month: u8, day: u8) -> String;
}
// Shim
#[cfg(engine)]
pub fn format_date(_year: u32, _month: u8, _day: u8) -> String {
    String::new()
}

#[cfg(engine)]
fn parse_date(date: String) -> (u32, u8, u8) {
    let date_parts = date.split('-').collect::<Vec<_>>();

    (
        date_parts[0]
            .parse()
            .expect("invalid year in post"),
        date_parts[1]
            .parse()
            .expect("invalid month in post"),
        date_parts[2]
            .parse()
            .expect("invalid day in post")
    )
}

/// Parses frontmatter and contents from a Markdown file. If `parse_contents` is `false`,
/// the contents will not be parsed, and an empty string will be returned instead.
#[cfg(engine)]
fn parse_frontmatter(file_contents: String, parse_contents: bool) -> (HashMap<String, String>, String) {
    let mut frontmatter = HashMap::new();
    let mut contents = String::new();

    let mut lines = file_contents.lines();
    // Advance into the frontmatter, which must be at the top
    if lines.next() != Some("---") {
        panic!("no frontmatter found in post");
    }

    // Parse frontmatter
    let mut in_frontmatter = true;
    for line in lines {
        if line == "---" {
            in_frontmatter = false;
            continue;
        }

        if in_frontmatter {
            if line.contains(":") {
                let line = line.split(':').collect::<Vec<_>>();
                let key = line[0].trim().to_string();
                let val = line[1].trim().to_string();

                frontmatter.insert(key, val);
            }
        } else {
            // Remaining contents of file
            contents.push_str(line);
            contents.push('\n');
        }
    }

    let contents = if parse_contents {
        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_STRIKETHROUGH);
        opts.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&contents, opts);
        let mut html_contents = String::new();
        html::push_html(&mut html_contents, parser);

        html_contents
    } else {
        String::new()
    };

    (frontmatter, contents)
}
