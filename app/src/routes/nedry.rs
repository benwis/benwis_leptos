use leptos::*;
use leptos_meta::*;

#[component]
pub fn Nedry() -> impl IntoView {
    view! {
        <Meta property="og:title" content="Ah-Ah-Ah You didn't say the magic word!"/>
        <Title text="Ah-Ah-Ah You didn't say the magic word!"/>
        <Meta name="description" content="Ah-Ah-Ah You didn't say the magic word!"/>
        <Meta property="og:description" content="Ah-Ah-Ah You didn't say the magic word!"/>
        <section class="pt-6">
            <img
                class="mx-auto"
                src="/img/Ahahah.webp"
                alt="Ah-Ah-Ah! You Didn't Say The Magic Word"
            />
            <h1 class="text-3xl text-black dark:text-white">
                "Ah-Ah-Ah! You Didn't Say The Magic Word!"
            </h1>
        </section>
    }
}
