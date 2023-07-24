use crate::functions::post::{get_posts, AddPost, DeletePost, UpdatePost};
use crate::models::Post;
use leptos::*;
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
            pub_date: post.created_at_pretty,
            author: post.user.unwrap_or_default().display_name,
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

pub fn generate_rss(title: &str, description: &str, link: &str, posts: Vec<Post>) -> String {
    let rss_entries = posts
        .into_iter()
        .map(|p| p.into())
        .map(|r: RssEntry| r.to_item())
        .collect::<String>();

    format!(
        r#"
        <xml version="1.0" encoding="UTF-8"/>
        <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
            <channel>
                <title>{title}</title>
                <description>{description}</description>
                <link>{link}</link>
                <language>"en-us"</language>
                <ttl>60</ttl>
                <atom:link href="https://benw.is/rss.xml" rel="self" type="application/rss+xml" />
                {}
            </channel>
        </rss>   
     "#,
        rss_entries
    )
}

#[component]
pub fn Rss() -> impl IntoView {
    let add_post = create_server_multi_action::<AddPost>();
    let update_post = create_server_action::<UpdatePost>();
    let delete_post = create_server_action::<DeletePost>();

    // list of posts is loaded from the server in reaction to changes
    let posts = create_resource(

        move || {
            (
                add_post.version().get(),
                update_post.version().get(),
                delete_post.version().get(),
            )
        },
        move |_| get_posts(),
    );

    view! {
        <Transition fallback=|| {
            view! {  "Loading" }
        }>
            {move || {
                let posts = {
                    move || {
                        posts
                            .read()
                            .map(|post| match post {
                                Ok(p) => p.into_iter().filter(|p| p.published).collect::<Vec<Post>>(),
                                Err(_) => Vec::new(),
                            })
                            .unwrap_or_default()
                    }
                };
                let rss = generate_rss(
                    "benwis Blog",
                    "The potentially misguided ramblings of a Rust developer flailing around on the web",
                    "http://benw.is",
                    posts(),
                );
                rss.into_view()
            }}
        </Transition>
    }
}
