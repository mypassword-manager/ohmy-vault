//! Minimal hash-based router.
//!
//! Routes look like https://USERNAME.github.io/ohmy-vault/#/signin
//! Because the part after # never reaches the server, GitHub Pages serves the
//! same index.html for every route. Direct visits and email links work without
//! any 404 workaround.
#![allow(dead_code)]

use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    Landing,
    SignIn,
    SignUp,
    VerifyEmail,
    ResetPassword,
    Dashboard,
    NotFound,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub route: Route,
    pub query: Vec<(String, String)>,
}

impl Location {
    pub fn param(&self, key: &str) -> Option<&str> {
        self.query
            .iter()
            .find(|(k, _)| k.as_str() == key)
            .map(|(_, v)| v.as_str())
    }
}

/// Parses a hash such as "#/reset?token=abc" into a route and query parameters.
pub fn parse_hash(raw: &str) -> Location {
    let raw = raw.trim_start_matches('#');
    let (path, query) = raw.split_once('?').unwrap_or((raw, ""));
    let path = path.trim_end_matches('/');

    let route = match path {
        "" => Route::Landing,
        "/signin" => Route::SignIn,
        "/signup" => Route::SignUp,
        "/verify" => Route::VerifyEmail,
        "/reset" => Route::ResetPassword,
        "/dashboard" => Route::Dashboard,
        _ => Route::NotFound,
    };

    let query: Vec<(String, String)> = query
        .split('&')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            Some((decode(key), decode(value)))
        })
        .collect();

    Location { route, query }
}

fn decode(value: &str) -> String {
    let replaced = value.replace('+', " ");
    js_sys::decode_uri_component(&replaced)
        .map(String::from)
        .unwrap_or(replaced)
}

fn current_hash() -> String {
    web_sys::window()
        .and_then(|window| window.location().hash().ok())
        .unwrap_or_default()
}

/// Changes the page address to the given hash path, for example "/dashboard".
pub fn navigate(path: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_hash(path);
    }
}

/// Creates the router signal, keeps it updated on hash changes, and provides it as context.
/// Call once from the root component.
pub fn provide_router() -> RwSignal<Location> {
    let location = create_rw_signal(parse_hash(&current_hash()));

    if let Some(window) = web_sys::window() {
        let on_change = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
            location.set(parse_hash(&current_hash()));
        });
        let _ = window
            .add_event_listener_with_callback("hashchange", on_change.as_ref().unchecked_ref());
        // The listener lives for the whole page lifetime, so leaking it is intentional.
        on_change.forget();
    }

    provide_context(location);
    location
}

/// Reads the router signal inside any child component.
pub fn use_location() -> RwSignal<Location> {
    expect_context::<RwSignal<Location>>()
}