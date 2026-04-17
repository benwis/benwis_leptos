use crate::functions::post::get_some_posts_meta;
use leptos::tachys::either::EitherOf3;
use leptos::{IntoView, component};
use leptos::{prelude::*, server::Resource, view};
use leptos_meta::*;

#[component]
pub fn Index() -> impl IntoView {
    // list of posts is loaded from the server in reaction to changes
    let posts_meta = Resource::new(move || (), move |_| get_some_posts_meta());

    view! {
        <Meta property="og:title" content="benwis" />
        <Title text="benwis" />
        <Meta name="description" content="Ben Wishovich's personal website" />
        <Meta property="og:description" content="Ben Wishovich's personal website" />
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg" />

        <div id="site-body">
            <div id="home">
                <div id="home__header">
                    <div id="home__header-body">
                        <div id="home__site-logo-type-wrap">
                            <svg
                                id="home__site-logo-type"
                                xmlns="http://www.w3.org/2000/svg"
                                xmlns:xlink="http://www.w3.org/1999/xlink"
                                x="0px"
                                y="0px"
                                viewBox="0 0 2205.8 143.7"
                                style="enable-background:new 0 0 2205.8 143.7;"
                                xml:space="preserve"
                            >
                                <g class="st0">
                                    <path
                                        class="st1"
                                        d="M121.6,107.9c0,23.7-18.8,33.4-47.7,33.4H0V2.9h68.8c29.7,0,48.7,9.4,48.7,32.3v8.4c0,13.9-7.2,22.1-19.4,26.2
                                        c14.7,3.7,23.5,12.7,23.5,29.9V107.9z M87.8,40.5c0-11.5-10-13.5-23.7-13.5H29.9V60h34c13.7,0,23.9-1.4,23.9-12.7V40.5z M91.9,95.8
                                        C91.9,83.1,81,81,68.6,81H29.9v35.6h38.9c13.1,0,23.1-2.7,23.1-14.3V95.8z"
                                    ></path>
                                    <path
                                        class="st1"
                                        d="M185.6,141.2V2.9H292c1,7,1,17.2,0,24.1h-76.5v32.5H290c1,7,1,17,0,23.7h-74.5v33.8H292c1,6.8,1,17.4,0,24.1
                                        H185.6z"
                                    ></path>
                                    <path
                                        class="st1"
                                        d="M385.6,45.2v96c-9.4,1-20.3,1-29.9,0V2.9C365.9,2,378,2,388.2,2.9l69,98.2V2.9c10.6-0.8,20.1-0.8,29.9,0v138.3
                                        c-11.1,0.8-22.3,0.8-33.4,0L385.6,45.2z"
                                    ></path>
                                    <path
                                        class="st1"
                                        d="M739.4,41.3l-31.3,99.9c-9.2,0.8-20.3,0.8-29.3,0.4l-45-138.8c8.6-0.8,22.3-0.8,30.5,0l29.9,99.9l29.1-99.9
                                        c8.8-0.8,24.6-0.8,32.9,0l28.7,99.5l30.3-99.5c8.6-0.8,21.3-0.8,29.7,0l-45.2,138.3c-9,0.8-19.9,0.8-28.9,0L739.4,41.3z"
                                    ></path>
                                </g>
                                <g class="st0">
                                    <path
                                        class="st2"
                                        d="M878.4,107.2c7.4-1,21.9-1,29.3,0v34c-7.4,1-21.9,1-29.3,0V107.2z"
                                    ></path>
                                </g>
                                <g class="st0">
                                    <path
                                        class="st1"
                                        d="M970.1,2.9c8.8-0.8,21.5-0.8,29.9,0v138.3c-8.4,0.8-21.1,0.8-29.9,0V2.9z"
                                    ></path>
                                    <path
                                        class="st1"
                                        d="M1176.4,110.7c0,23.9-22.7,32.9-60.4,32.9c-15.3,0-34.8-0.8-52.6-3.3c-1.2-6.8-1.2-17.8,0-24.8
                                        c20.3,2.3,36.8,3.3,52.2,3.3c17.2,0,31.3-1.6,31.3-12.7v-7.2c0-7.6-3.3-8.2-19.9-12.3l-35.6-8.6c-17-4.1-28.4-11.5-28.4-31.9V32.9
                                        c0-24.1,25-32.9,61.2-32.9c16.8,0,31.3,0.8,50.8,4.3c1.2,6.8,1.2,17.2,0,23.9c-20.1-2.7-33.6-3.5-49.9-3.5
                                        c-18.8,0-32.5,1.4-32.5,12.5v5.5c0,8,4.9,9.8,24.6,14.7l31.1,7.6c17.8,4.5,28.2,10,28.2,30.7V110.7z"
                                    ></path>
                                </g>
                                <g class="st0">
                                    <path
                                        class="st2"
                                        d="M1339.1,83.7h-69v57.5c-8.4,0.8-21.1,0.8-29.9,0V2.9c9.2-0.8,21.3-0.8,29.9,0v54.6h69V2.9
                                        c8.6-0.8,21.5-0.8,29.9,0v138.3c-8.4,0.8-20.9,0.8-29.9,0V83.7z"
                                    ></path>
                                    <path
                                        class="st2"
                                        d="M1565,104.6c0,29.3-26.2,39.1-65.5,39.1c-39.5,0-65.5-9.8-65.5-39.1V39.1C1434,10,1460,0,1499.5,0
                                        c39.3,0,65.5,10,65.5,39.1V104.6z M1535.1,44.8c0-17.8-16-19.9-35.6-19.9s-35.6,2-35.6,19.9v53.6c0,17.6,16,19.9,35.6,19.9
                                        s35.6-2.3,35.6-19.9V44.8z"
                                    ></path>
                                    <path
                                        class="st2"
                                        d="M1610.3,2.9c9-0.8,22.3-0.8,31.1,0l39.9,100.3l39.3-100.3c8.6-0.8,21.9-0.8,30.1,0l-55.5,138.3
                                        c-8.8,0.8-19.4,0.8-28,0L1610.3,2.9z"
                                    ></path>
                                    <path
                                        class="st2"
                                        d="M1800.8,2.9c8.8-0.8,21.5-0.8,29.9,0v138.3c-8.4,0.8-21.1,0.8-29.9,0V2.9z"
                                    ></path>
                                    <path
                                        class="st2"
                                        d="M1896.8,38.5c0-29.1,26.2-38.5,66.5-38.5c15.3,0,30.1,1,47.9,3.9c1,6.8,1,17,0,23.7c-15.1-1.8-28.7-2.7-43-2.7
                                        c-23.5,0-41.8,1.4-41.8,19.6v54c0,18,18.2,19.6,41.8,19.6c14.1,0,29.9-2,43.4-4.5c1.8,7.2,2.7,15.8,2.7,23.7
                                        c-18.4,4.1-37,6.1-51,6.1c-40.3,0-66.5-9.2-66.5-38.3V38.5z"
                                    ></path>
                                    <path
                                        class="st2"
                                        d="M2175.9,83.7h-69v57.5c-8.4,0.8-21.1,0.8-29.9,0V2.9c9.2-0.8,21.3-0.8,29.9,0v54.6h69V2.9
                                        c8.6-0.8,21.5-0.8,29.9,0v138.3c-8.4,0.8-20.9,0.8-29.9,0V83.7z"
                                    ></path>
                                </g>
                            </svg>
                        </div>
                        <svg
                            id="home__site-logo-mark"
                            xmlns="http://www.w3.org/2000/svg"
                            xmlns:xlink="http://www.w3.org/1999/xlink"
                            x="0px"
                            y="0px"
                            viewBox="0 0 2265.2 864"
                            style="enable-background:new 0 0 2265.2 864;"
                            xml:space="preserve"
                        >
                            <polygon
                                class="st0"
                                points="1023.2,0 663.2,0 924.2,864 1132.9,363.1 "
                            ></polygon>
                            <polygon
                                class="st0"
                                points="1644.2,0 1435.5,500.9 1545.2,864 1753.9,363.1 "
                            ></polygon>
                            <path
                                class="st0"
                                d="M569.6,391.7c26.6-40,42.2-87.9,42.2-139.6C611.8,113,499.1,0.3,360,0.2V0H0v864h96h264h96v-0.4
                                c135.1-4.6,243.2-115.5,243.2-251.7C699.2,517.2,646.9,434.7,569.6,391.7z M304.1,718C245.5,718,198,670.5,198,611.9
                                s47.5-106.1,106.1-106.1s106.1,47.5,106.1,106.1S362.7,718,304.1,718z M410.2,252.1c0,58.6-47.5,106.1-106.1,106.1
                                c-58.6,0-106.1-47.5-106.1-106.1c0-58.6,47.5-106.1,106.1-106.1C362.7,146,410.2,193.5,410.2,252.1z"
                            ></path>
                            <ellipse
                                transform="matrix(0.9979 -6.525722e-02 6.525722e-02 0.9979 -39.284 21.1501)"
                                class="st1"
                                cx="304.1"
                                cy="611.9"
                                rx="106.1"
                                ry="106.1"
                            ></ellipse>
                            <ellipse class="st1" cx="304.1" cy="252.1" rx="106.1" ry="106.1"></ellipse>
                            <polygon
                                class="st1"
                                points="1905.2,0 1753.9,363.1 1545.2,864 1905.2,864 2265.2,0 "
                            ></polygon>
                            <polygon
                                class="st1"
                                points="1284.2,0 1132.9,363.1 924.2,864 1284.2,864 1435.5,500.9 1644.2,0 "
                            ></polygon>
                        </svg>
                    </div>
                </div>

                <div id="home__about">
                    <div id="home__about-body">
                        <div id="home__about-rule"></div>
                        <div id="home__about-layout">
                            <div id="home__about-summary">
                                "Software Engineer " <br /> "Full Stack Web Developer " <br />
                                "Runner " <br /> "Rust " <br /> "Typescript " <br /> "WASM " <br />
                                "Python " <br /> "React " <br /> "Svelte " <br />
                            </div>
                            <div id="home__about-pagination">
                                <a class="button" href="/about">"About"</a>
                            </div>
                        </div>
                    </div>
                </div>

                <div id="home__blog">
                    <div id="home__blog-layout">
                        <div id="home__blog-posts-wrapper">
                            <div id="home__blog-posts">
                                <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                                    {move || Suspend::new(async move {
                                        match posts_meta.await {
                                            Err(e) => {
                                                EitherOf3::A(view! {
                                                    <pre class="error">"Server Error: " {e.to_string()}</pre>
                                                })
                                            }
                                            Ok(posts) => {
                                                if posts.is_empty() {
                                                    EitherOf3::B(view! {
                                                        <p>"No posts were found."</p>
                                                    })
                                                } else {
                                                    EitherOf3::C(
                                                        posts
                                                            .into_iter()
                                                            .filter(|p| p.published)
                                                            .take(2)
                                                            .map(|post_meta| {
                                                                let hero = post_meta.hero.unwrap_or_default();
                                                                view! {
                                                                    <a
                                                                        class="post-card"
                                                                        href=format!("/posts/{}", post_meta.slug)
                                                                    >
                                                                        <img
                                                                            class="post-card__underlay"
                                                                            width="650"
                                                                            src=hero
                                                                        />
                                                                        <div class="post-card__layout">
                                                                            <h3 class="post-card__heading">{post_meta.title}</h3>
                                                                            <p class="post-card__meta">
                                                                                {post_meta.created_at_pretty}
                                                                            </p>
                                                                        </div>
                                                                    </a>
                                                                }
                                                            })
                                                            .collect::<Vec<_>>(),
                                                    )
                                                }
                                            }
                                        }
                                    })}
                                </Suspense>
                            </div>
                        </div>
                        <div id="home__blog-pagination">
                            <a class="button" href="/posts">"Blog"</a>
                        </div>
                    </div>
                    <div id="home__blog-rule"></div>
                </div>

                <div id="home__portfolio">
                    <div id="home__portfolio-rule"></div>
                    <div id="home__portfolio-layout">
                        <div id="home__portfolio-posts">
                            <a class="post-card" href="https://github.com/leptos-rs/leptos">
                                <div class="post-card__layout">
                                    <h3 class="post-card__heading">"Leptos"</h3>
                                    <p class="post-card__meta">
                                        "Lead Maintainer Rust full-stack web framework"
                                    </p>
                                </div>
                            </a>
                            <a class="post-card" href="https://github.com/benwis/tower-governor">
                                <div class="post-card__layout">
                                    <h3 class="post-card__heading">"tower-governor"</h3>
                                    <p class="post-card__meta">"Rust API rate limiting crate"</p>
                                </div>
                            </a>
                            <a class="post-card" href="https://github.com/benwis/femark">
                                <div class="post-card__layout">
                                    <h3 class="post-card__heading">"femark"</h3>
                                    <p class="post-card__meta">
                                        "Rust Markdown to HTML compiler crate with syntax highlighting"
                                    </p>
                                </div>
                            </a>
                            <a class="post-card" href="https://praxiscycles.com">
                                <div class="post-card__layout">
                                    <h3 class="post-card__heading">"Praxis Cycles"</h3>
                                    <p class="post-card__meta">
                                        "Custom ecommerce store built on Wordpress"
                                    </p>
                                </div>
                            </a>
                        </div>
                        <div id="home__portfolio-pagination">
                            <a class="button" href="/portfolio">"Portfolio"</a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
