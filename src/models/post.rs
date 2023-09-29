use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use octocrab::{models::repos::Content, Octocrab};
    use femark::{process_markdown_to_html_with_frontmatter, HTMLOutput};
    use parking_lot::RwLock;
    use crate::errors::BenwisAppError;
    use std::sync::Arc;
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostFrontmatter {
    title: String,
    slug: String,
    excerpt: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    published: bool,
    preview: bool,
    tags: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub toc: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published: bool,
    pub preview: bool,
    pub links: Option<String>,
    pub tags: Vec<String>,
}
cfg_if! {
if #[cfg(feature = "ssr")] {
impl TryFrom<String> for Post {
    type Error = BenwisAppError;
    fn try_from(post_string: String) -> Result<Self, Self::Error> {
        let HTMLOutput {
            content,
            toc,
            frontmatter,
            ..
        } = process_markdown_to_html_with_frontmatter(&post_string, true).unwrap_or_default();

        let frontmatter = match frontmatter {
                    Some(f) => f,
                    None => return Err(BenwisAppError::MissingOrInvalidFrontmatter)
                };
        let code_block = frontmatter.code_block.ok_or(BenwisAppError::MissingOrInvalidFrontmatter)?;
        let fm: PostFrontmatter = toml::from_str(&code_block.source)?;

        Ok(Self {
            title: fm.title,
            slug: fm.slug,
            excerpt: Some(fm.excerpt),
            content,
            toc,
            created_at: fm.created_at,
            updated_at: fm.updated_at,
            published: fm.published,
            preview: fm.preview,
            links: None,
            tags: fm.tags,
        })
    }
}
        }
    }

cfg_if! {
if #[cfg(feature = "ssr")] {
#[derive(Clone, Debug, Default)]
pub struct PostsContainer(pub Arc<RwLock<Posts>>);

impl PostsContainer {
    pub async fn new_with_posts() -> Result<Self, BenwisAppError> {
        let mut posts = Posts::default();
        posts.fetch_posts_from_github().await?;
        posts.posts.sort_unstable_by(|_a, b, _c, d| d.created_at.partial_cmp(&b.created_at).unwrap());

        let container = PostsContainer(Arc::new(RwLock::new(posts)));
        Ok(container)
    }
}
}
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Posts {
    pub posts: IndexMap<String, Post>,
    pub last_checked: DateTime<Utc>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
impl Posts {
    /// If we have no posts, fetch them from the github repo
    pub async fn fetch_posts_from_github(&mut self) -> Result<(), BenwisAppError> {
        let api_token = std::env::var("GITHUB_TOKEN").expect("Failed to get Github Token");
        let octocrab = Octocrab::builder()
            .personal_token(api_token)
            .build()
            .unwrap();
        let content = octocrab
            .repos("benwis", "benwis_posts")
            .get_content()
            .path("posts")
            .send()
            .await
            .unwrap();

        //content.items.iter().for_each(|i| println!("{}", &i.path));
        // Get the path to all the post folders
        let content_paths: Vec<Content> = content
            .items
            .into_iter()
            .filter(|i| i.path.starts_with("posts/"))
            .collect();

        //Iterate over all the post folders
        for content in content_paths {
            let mut post_contents = octocrab
                .repos("benwis", "benwis_posts")
                .get_content()
                .path(content.path)
                .send()
                .await
                .unwrap();

            //Filter out all paths that do not end with .md
            post_contents.items.retain(|p| p.path.ends_with(".md"));

            let contents = post_contents.take_items();
            for item in contents {
                // Fetch it again, because it won't be included in a multi file response
                // Not really sure why tbh
                let mut content = octocrab
                    .repos("benwis", "benwis_posts")
                    .get_content()
                    .path(item.path)
                    .send()
                    .await
                    .unwrap();
                let contents = content.take_items();
                let c = &contents[0];
                let decoded_content = c.decoded_content().unwrap();

                let post: Post = decoded_content.try_into()?;

                self.posts.insert(post.slug.clone(), post);
            }
        }
        Ok(())
    }
    /// Get the post. If it's not in local, check github. If not in Github, return None
    pub async fn fetch_post_from_github(&mut self, slug: &str) -> Result<Option<Post>, BenwisAppError> {
        let api_token = std::env::var("GITHUB_TOKEN").expect("Failed to get Github Token");
        let octocrab = Octocrab::builder()
            .personal_token(api_token)
            .build()
            .unwrap();

        let post_path = format!("posts/{}/{}.md", slug, slug);
         let mut content =  match octocrab
                    .repos("benwis", "benwis_posts")
                    .get_content()
                    .path(post_path)
                    .send()
                    .await
                    {
                        Ok(p) => p,
                        Err(_) => return Ok(None),
                    };
                let contents = content.take_items();
                let c = &contents[0];
                let decoded_content = c.decoded_content().unwrap();

                let post: Post = decoded_content.try_into()?;

                self.posts.insert(post.slug.clone(), post.clone());
                Ok(Some(post))
    }
}
}
    }
