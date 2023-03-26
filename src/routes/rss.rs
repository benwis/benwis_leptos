use leptos::*;
use crate::functions::post::{get_posts, AddPost,UpdatePost, DeletePost};
use crate::models::Post;
// export type RssEntry = {
//     title: string;
//     link: string;
//     description: string | null;
//     pubDate: string;
//     author?: string;
//     guid?: string;
//   };
  
  pub struct RssEntry{
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub pub_date: String,
    pub author: String,
    pub guid: String
  }

  impl From<Post> for RssEntry{
    fn from(post: Post) -> Self{
        let full_url = format!("https://benw.is/posts/{}", post.slug);
        Self{
            title: post.title,
            link: full_url.clone(),
            description: post.excerpt,
            pub_date: post.created_at_pretty,
            author: post.user.unwrap_or_default().display_name,
            guid: full_url
        }
    }
  }
//   export function generateRss({
//     description,
//     entries,
//     link,
//     title,
//   }: {
//     title: string;
//     description: string;
//     link: string;
//     entries: RssEntry[];
    
//   }): string {
//     return `<?xml version="1.0" encoding="UTF-8"?>
//   <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
//     <channel>
//       <title>${title}</title>
//       <description>${description}</description>
//       <link>${link}</link>
//       <language>en-us</language>
//       <ttl>60</ttl>
//       <atom:link href="https://benw.is/rss.xml" rel="self" type="application/rss+xml" />
//       ${entries
//         .map(
//           (entry) => `
//         <item>
//           <title><![CDATA[${entry.title}]]></title>
//           <description><![CDATA[${entry.description}]]></description>
//           <pubDate>${entry.pubDate}</pubDate>
//           <link>${entry.link}</link>
//           ${entry.guid ? `<guid isPermaLink="false">${entry.guid}</guid>` : ""}
//         </item>`
//         )
//         .join("")}
//     </channel>
//   </rss>`;
//   }

pub fn generate_rss(cx: Scope, description: String, posts: Vec<Post>, link: String, title: String ){

    let entries: Vec<RssEntry> = posts.into_iter().map(|p| p.into()).collect();

     // list of posts is loaded from the server in reaction to changes
     let rss_shell = r#"
        <xml version="1.0" encoding="UTF-8"/>
        <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
            <channel>
                <title>{title}</title>
                <description>{description}</description>
                <link>{link}</link>
                <language>"en-us"</language>
                <ttl>60</ttl>
                <atom:link href="https://benw.is/rss.xml" rel="self" type="application/rss+xml" />
            </channel>
        </rss>   
     "#;
     let entries_block: String = posts;

    view!{cx,
        <xml version="1.0" encoding="UTF-8"/>
        <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
          <channel>
            <title>{title}</title>
            <description>{description}</description>
            <link>{link}</link>
            <language>"en-us"</language>
            <ttl>60</ttl>
            <atom:link href="https://benw.is/rss.xml" rel="self" type="application/rss+xml" />   
            {entries
              .map(
                |entry| {view!{cx,
              <item>
                <title><![CDATA[${entry.title}]]></title>
                <description><![CDATA[${entry.description}]]></description>
                <pubDate>${entry.pubDate}</pubDate>
                <link>{entry.link}</link>
                ${entry.guid ? `<guid isPermaLink="false">${entry.guid}</guid>` : ""}
              </item>
            }
        }
              )
            }
          </channel>
        </rss>
    }

}

#[component]
pub fn Rss(cx: Scope) -> impl IntoView{
    let add_post = create_server_multi_action::<AddPost>(cx);
    let update_post = create_server_action::<UpdatePost>(cx);
    let delete_post = create_server_action::<DeletePost>(cx);

    view!{cx,
        
     
  export function generateRss({
    description,
    entries,
    link,
    title,
  }: {
    title: string;
    description: string;
    link: string;
    entries: RssEntry[];
  }): string {
    return `
    <?xml version="1.0" encoding="UTF-8"?>
  <rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
    <channel>
      <title>${title}</title>
      <description>${description}</description>
      <link>${link}</link>
      <language>en-us</language>
      <ttl>60</ttl>
      <atom:link href="https://benw.is/rss.xml" rel="self" type="application/rss+xml" />
      ${entries
        .map(
          (entry) => `
        <item>
          <title><![CDATA[${entry.title}]]></title>
          <description><![CDATA[${entry.description}]]></description>
          <pubDate>${entry.pubDate}</pubDate>
          <link>${entry.link}</link>
          ${entry.guid ? `<guid isPermaLink="false">${entry.guid}</guid>` : ""}
        </item>`
        )
        .join("")}
    </channel>
  </rss>`;
  }
    }
}