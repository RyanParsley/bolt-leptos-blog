use std::fs;
use std::path::Path;
use gray_matter::Matter;
use pulldown_cmark::{Parser, html::push_html, Options};
use chrono::NaiveDate;

fn main() {
    // Create the generated directory if it doesn't exist
    let out_dir = Path::new("src/generated");
    fs::create_dir_all(out_dir).unwrap();

    let matter = Matter::new();
    let posts_dir = Path::new("content/posts");
    let mut posts = Vec::new();

    // Process each markdown file
    for entry in fs::read_dir(posts_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().unwrap_or_default() == "md" {
            let content = fs::read_to_string(&path).unwrap();
            let matter_result = matter.parse(&content);
            
            if let Some(front_matter) = matter_result.data {
                let title = front_matter["title"].as_str().unwrap().to_string();
                let author = front_matter["author"].as_str().unwrap().to_string();
                let date = front_matter["date"].as_str().unwrap();
                let description = front_matter["description"].as_str().unwrap().to_string();
                
                // Convert markdown to HTML
                let mut html = String::new();
                let parser = Parser::new_ext(&matter_result.content, Options::all());
                push_html(&mut html, parser);

                // Generate slug from filename
                let slug = path.file_stem().unwrap().to_str().unwrap().to_string();

                posts.push(format!(
                    r#"BlogPost::new(
                        "{}".to_string(),
                        "{}".to_string(),
                        NaiveDate::parse_from_str("{}", "%Y-%m-%d").unwrap(),
                        "{}".to_string(),
                        r#"{}"#.to_string(),
                        "{}".to_string(),
                    )"#,
                    title, author, date, description, html, slug
                ));
            }
        }
    }

    // Generate posts.rs with all blog posts
    let posts_rs = format!(
        r#"use chrono::NaiveDate;
use crate::types::post::BlogPost;

pub fn get_posts() -> Vec<BlogPost> {{
    vec![
        {}
    ]
}}"#,
        posts.join(",\n        ")
    );

    fs::write(
        out_dir.join("posts.rs"),
        posts_rs
    ).unwrap();
}