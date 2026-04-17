use leptos::prelude::*;
use leptos::{IntoView, component, view};
use leptos_meta::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Meta property="og:title" content="About Me" />
        <Title text="About Me" />
        <Meta name="description" content="A page describing me" />
        <Meta property="og:description" content="A page describing me" />
        <Meta property="og:image" content="https://benwis.imgix.net/pictureofMe.jpeg" />

        <div id="page">
            <div id="page__header" class="align-center">
                <h1 id="page__heading">"About"</h1>
            </div>
            <div id="page__layout">
                <div id="page__body">
                    <img
                        class="rounded-sm"
                        src="/img/ben_catcarbon.png"
                        alt="A white guy with blue eyes, dark hair, and glasses. Kinda looks like Harry Potter. Describing yourself is hard"
                    />
                    <section>
                        <p>
                            "I'm a Software Engineer and Full Stack Web Developer, living and working in the SF Bay Area. I graduated from San Jose State with a degree in Industrial and Systems Engineering, and then made the jump into software engineering and web development."
                        </p>
                        <br />
                        <p>
                            "I've been coding and building things since High School, and have helped build a variety of projects. Everything from mapping software for UAVs, motor controllers for automated vending machines, to electric motorcycle diagnostic software."
                        </p>
                        <br />
                        <p>
                            "I mostly work on Rust on the web, maintaining several crates to that purpose, including Leptos, tower-governor, and femark. The future is bright, and I'm excited to help build a better web"
                        </p>
                        <br />
                        <p>
                            "In my free time, I enjoy running, cooking all manner of delicious foods, and hot sauces."
                        </p>
                        <br />
                        <address>
                            <section>
                                <h3>"Contact Me:"</h3>
                                <a href="mailto:ben@benw.is">"Email"</a>
                                <br />
                                <a href="https://hachyderm.io/@benwis">"Mastodon"</a>
                                <br />
                                <a href="https://www.linkedin.com/in/benwis">"Linkedin"</a>
                                <br />
                                <a href="https://twitter.com/iambenwis">"Twitter"</a>
                            </section>
                        </address>
                    </section>
                </div>
            </div>
        </div>
    }
}
