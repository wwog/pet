use dioxus::prelude::*;

use crate::auth::session::{restore_session, AuthState, RestoreOutcome, Session};
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
    let mut session = use_context_provider(Session::new);

    // 在根组件启动一次 token restore,无论用户从哪个 URL 进入都会执行。
    // 避免"直接访问 /briefing 绕过 Splash"导致登录态未恢复。
    use_effect(move || {
        spawn(async move {
            match restore_session().await {
                Some(RestoreOutcome::Authed { access, refresh, expires_at, user }) => {
                    session.state.set(AuthState::Authenticated {
                        access_token: access,
                        user: crate::api::types::UserInfo {
                            user_id: user.user_id,
                            account: user.account,
                            nickname: user.nickname,
                            avatar: user.avatar,
                            role: user.role,
                        },
                        refresh_token: refresh,
                        expires_at,
                    });
                }
                None => {
                    session.set_guest();
                }
            }
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
