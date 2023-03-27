use leptos::*;

#[component]
pub fn Nedry(cx: Scope) -> impl IntoView{
    view!{cx,
        <section class="pt-6">
        <img class="mx-auto" src="/img/Ahahah.webp" alt="You didn't say the magic word"/>
        <h1 class="text-3xl text-black dark:text-white">"You Didn't Say The Magic Word!"</h1>
        </section>
    }
}