use dioxus::prelude::*;

use crate::api::types::UserInfo;
use crate::app::Route;
use crate::auth::session::{restore_session, AuthState, RestoreOutcome, Session};

#[component]
pub fn Splash() -> Element {
    let mut session = use_context::<Session>();

    use_effect(move || {
        spawn(async move {
            let nav = navigator();
            match restore_session().await {
                Some(RestoreOutcome::Authed { access, refresh, expires_at, user }) => {
                    session.state.set(AuthState::Authenticated {
                        access_token: access,
                        user: UserInfo {
                            user_id: user.user_id,
                            account: user.account,
                            nickname: user.nickname,
                            avatar: user.avatar,
                            role: user.role,
                        },
                        refresh_token: refresh,
                        expires_at,
                    });
                    nav.replace(Route::Briefing {});
                }
                None => {
                    session.set_guest();
                    nav.replace(Route::Login {});
                }
            }
        });
    });

    rsx! {
        div { class: "splash-screen",
            div { class: "splash-logo",
                svg { width: "80", height: "80", view_box: "0 0 26 26",
                    circle { cx: "13", cy: "15", r: "8", fill: "var(--accent)" }
                    circle { cx: "13", cy: "11", r: "6", fill: "var(--accent)" }
                    ellipse { cx: "8", cy: "8", rx: "2.5", ry: "4", fill: "var(--accent)",
                        transform: "rotate(-20 8 8)" }
                    ellipse { cx: "18", cy: "8", rx: "2.5", ry: "4", fill: "var(--accent)",
                        transform: "rotate(20 18 8)" }
                    circle { cx: "11", cy: "11", r: "1", fill: "#fff" }
                    circle { cx: "15", cy: "11", r: "1", fill: "#fff" }
                }
            }
        }
    }
}
