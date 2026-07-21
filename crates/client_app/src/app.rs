use dioxus::prelude::*;

use crate::auth::session::Session;
use crate::components::app_layout::AppLayout;
use crate::routes::ai_chat::AiChat;
use crate::routes::briefing::Briefing;
use crate::routes::login::Login;
use crate::routes::me::Me;
use crate::routes::register::Register;
use crate::routes::splash::Splash;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Splash {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},

    #[layout(AppLayout)]
    #[route("/briefing")]
    Briefing {},
    #[route("/ai")]
    AiChat {},
    #[route("/me")]
    Me {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    // 提供全局 Session
    use_context_provider(|| Session::new());

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
