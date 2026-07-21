mod api;
mod app;
mod auth;
mod components;
mod routes;
mod state;

fn main() {
    dioxus::launch(app::App);
}
