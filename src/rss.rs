use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
use crate::models::Post;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use indexmap::IndexMap;
use http::{HeaderValue, HeaderMap};
// export type RssEntry = {
//     title: string;
//     link: string;
//     description: string | null;
//     pubDate: string;
//     author?: string;
//     guid?: string;
//   };

pub struct RssEntry {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub pub_date: String,
    pub author: String,
    pub guid: String,
}

impl From<Post> for RssEntry {
    fn from(post: Post) -> Self {
        let full_url = format!("https://benw.is/posts/{}", post.slug);
        Self {
            title: post.title,
            link: full_url.clone(),
            description: post.excerpt,
            pub_date: post.created_at.to_rfc2822(),
            author: "benwis".to_string(),
            guid: full_url,
        }
    }
}

impl RssEntry {
    // Converts an RSSEntry to a String containing the rss item tags
    pub fn to_item(&self) -> String {
        format!(
r#"
        <item>
            <title><![CDATA[{}]]></title>
            <description><![CDATA[{}]]></description>
            <pubDate>{}</pubDate>
            <link>{}</link>
            <guid isPermaLink="false">{}</guid>
        </item>
      "#,
            self.title,
            self.description.clone().unwrap_or_default(),
            self.pub_date,
            self.guid,
            self.guid
        )
    }
}

pub fn generate_rss(
    title: &str,
    description: &str,
    link: &str,
    posts: &IndexMap<String, Post>,
) -> String {
    let rss_entries = posts
        .clone()
        .into_values()
        .filter(|p| p.published)
        .map(|p| p.into())
        .map(|r: RssEntry| r.to_item())
        .collect::<String>();

    format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
        <title>{title}</title>
        <description>{description}</description>
        <link>{link}</link>
        <language>en-us</language>
        <ttl>60</ttl>
        <atom:link href="https://benw.is/rss.xml" rel="self" type="application/rss+xml" />
        {}
    </channel>
</rss>   
     "#,
        rss_entries
    )
}
pub async fn rss_page(State(app_state): State<AppState>) -> impl IntoResponse {
    // list of posts is loaded from the server in reaction to changes
    let raw_posts = app_state.posts;
    let reader = raw_posts.0.read();
    let rss = generate_rss(
        "benwis Blog",
        "The potentially misguided ramblings of a Rust developer flailing around on the web",
        "http://benw.is",
        &reader.posts,
    );
    // Cache the thing
    let mut headers = HeaderMap::new();
    headers.insert(http::header::CACHE_CONTROL, HeaderValue::from_str("private, max-age=3600").unwrap());
            (headers,rss)
}
        }
    }
