mod app;
mod components;
mod config;
mod content;
mod pages;
mod router;

use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}