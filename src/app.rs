use crate::pages::{landing::LandingPage, not_found::NotFoundPage};
use crate::router::{provide_router, use_location, Route};
use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};

/// Root component. Part 3 and Part 4 will replace this file to add the sign-in,
/// sign-up, verification, reset and dashboard routes.
#[component]
pub fn App() -> impl IntoView {
    provide_router();
    install_flashlight();
    let location = use_location();

    view! {
        <div class="app-shell">
            <div class="flashlight" aria-hidden="true"></div>
            <main class="page">
                {move || match location.get().route {
                    Route::Landing => view! { <LandingPage/> }.into_view(),
                    _ => view! { <NotFoundPage/> }.into_view(),
                }}
            </main>
        </div>
    }
}

/// Stores the mouse position in CSS variables so the flashlight gradient can follow it.
fn install_flashlight() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(root) = window.document().and_then(|document| document.document_element()) else {
        return;
    };
    let root: web_sys::HtmlElement = root.unchecked_into();

    let on_move = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(
        move |event: web_sys::MouseEvent| {
            let style = root.style();
            let _ = style.set_property("--mouse-x", &format!("{}px", event.client_x()));
            let _ = style.set_property("--mouse-y", &format!("{}px", event.client_y()));
        },
    );
    let _ = window.add_event_listener_with_callback("mousemove", on_move.as_ref().unchecked_ref());
    on_move.forget();
}