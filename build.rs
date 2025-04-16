use maud::{html, Markup, PreEscaped};
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind::NotFound;
use std::path::Path;

const SRC_POSTS_DIR: &'static str = "pages/posts";
const DEST_POSTS_DIR: &'static str = "static/posts";
const STATIC_DIR: &'static str = "static";

#[derive(Serialize, Deserialize)]
struct PostMeta {
    date: String,
    tags: Vec<String>,
}

fn sync_post_meta(post: &str) -> PostMeta {
    let path = Path::new(SRC_POSTS_DIR).join(format!("{}.json", post));
    let result: PostMeta = match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            if e.kind() == NotFound {
                let meta = PostMeta {
                    date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                    tags: vec![],
                };
                write_post_meta(post, &meta);
                return meta;
            }
            panic!("Failed to read metadata for post");
        }
    };
    result
}

fn write_post_meta(post: &str, meta: &PostMeta) {
    let content = serde_json::to_string_pretty(&meta).unwrap();
    let path = Path::new(SRC_POSTS_DIR).join(format!("{}.json", post));
    fs::write(&path, content).expect("Failed to write metadata file");
}

fn tags_list(tags: &Vec<String>) -> Markup {
    html! {
        @if !tags.is_empty() {
            p class="tags" {
                @for tag in tags {
                    a class="tag" href={"/?tag="(tag)} {(tag)}
                    @if tag != tags.last().unwrap() {
                        span { ", " }
                    }
                }
            }
        }
    }
}
fn post_item(
    title: &str,
    handle: &str,
    display_date: &str,
    date: &str,
    tags: &Vec<String>,
) -> Markup {
    html! {
        li data-tags=(tags.join(",")) data-handle=(handle) data-date=(date) {
            div class="title-container" {
                div { a class="title" href={"/posts/"(handle)".html"} {(title)}}
                div {span class="date" { (display_date) } }
            }
            (tags_list(tags))
        }
    }
}

fn head_tags() -> Markup {
    html! {
        link rel="preconnect" href="https://fonts.googleapis.com" {}
        link rel="preconnect" href="https://fonts.gstatic.com" crossorigin {}
        link href="https://fonts.googleapis.com/css2?family=Castoro:ital@0;1&display=swap" rel="stylesheet" {}
        link rel="stylesheet" href="/styles.css" {}
        meta http-equiv="Content-Type" content="text/html; charset=UTF-8" {}
        meta name="viewport" content="width=device-width, initial-scale=1" {}
        script src="/index.js" defer {}
    }
}

fn page(title: &str, content: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                (head_tags())
                title { (title) }
            }
            body {
                div class="main" {
                    header {
                        a class="nav-link" href="/" { "POSTS" }
                        a class="nav-link" href="/about.html" { "ABOUT" }
                    }
                    div class="content" {
                        div class="sidebar" { }
                        div class="page-content" {
                            (content)
                        }
                    }
                    div class="footer" {
                        p {
                            "Built with "
                            a href="https://www.rust-lang.org/" {"Rust"}
                            " and a dash of "
                            a href="https://maud.lambda.xyz/" {"Maud"}
                            ". Brought to you by "
                            a href="https://github.com/tokio-rs/axum" {"Axum"}"."
                            a href="https://github.com/fo-nicks/nicksknacks.me" { img class="inline-icon" src="/img/github-mark.svg" {} }
                        }
                    }

                }
            }
        }
    }
}

fn blog_page(title: &str, content: &str, display_date: &str, tags: &Vec<String>) -> Markup {
    page(
        title,
        html! {
            (PreEscaped(content))
            footer {
                p class="date" { "Posted on " (display_date) }
                (tags_list(tags))
            }
        },
    )
}

fn extract_first_heading(markdown: &str) -> Option<String> {
    let parser = Parser::new(markdown);

    let mut in_heading = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
            }
            Event::Text(text) if in_heading => {
                return Some(text.to_string());
            }
            _ => {}
        }
    }

    None
}

fn generate_page(title: &str, path: &str) {
    let markdown =
        fs::read_to_string(format!("pages/{}.md", path)).expect("Failed to read Markdown file");
    let parser = Parser::new(&markdown);

    let output_path = Path::new(STATIC_DIR).join(format!("{}.html", path));
    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);
    let output = page(
        title,
        html! {
            (PreEscaped(&html_output))
        },
    );
    fs::write(output_path, output.into_string()).expect("Failed to write HTML file");
}

fn main() {
    fs::create_dir_all(SRC_POSTS_DIR).expect("Failed to create static directory");
    let mut post_items = String::new();
    for entry in fs::read_dir(SRC_POSTS_DIR).expect("Failed to read blog directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let meta = sync_post_meta(file_name);
            let output_path = Path::new(DEST_POSTS_DIR).join(format!("{}.html", file_name));
            let markdown = fs::read_to_string(&path).expect("Failed to read Markdown file");

            let mut options = Options::empty();
            options.insert(Options::ENABLE_SMART_PUNCTUATION);
            let parser = Parser::new_ext(&markdown, options);
            let title = extract_first_heading(&markdown)
                .unwrap_or_else(|| panic!("Failed to extract heading from {}", file_name));

            let display_date = chrono::NaiveDate::parse_from_str(&meta.date, "%Y-%m-%d")
                .expect("Failed to parse date")
                .format("%B %d, %Y")
                .to_string();
            post_items += post_item(&title, file_name, &display_date, &meta.date, &meta.tags)
                .into_string()
                .as_str();

            let mut html_output = String::new();

            html::push_html(&mut html_output, parser);
            html_output = blog_page(&title, &html_output, &display_date, &meta.tags).into_string();
            fs::write(output_path, html_output).expect("Failed to write HTML file");
        }
    }
    let index = page(
        "Posts",
        html! {
            ul class="post-list" {
                (PreEscaped(post_items))
            }
        },
    );
    fs::write(Path::new("static").join("index.html"), index.into_string())
        .expect("Failed to write index file");
    generate_page("About", "about");
}
