use crate::components::mascot::Mascot;
use leptos::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <section class="centered-panel">
            <Mascot mood="thinking"/>
            <h1>"Page not found"</h1>
            <p class="lead">"This page is not available yet, or the link is incorrect."</p>
            <a class="btn btn-primary" href="#/">"Back to home"</a>
        </section>
    }
}