use dioxus::prelude::*;

use super::status_bar::StatusBar;
use super::tab_bar::{TabBar, TabId};
use crate::app::Route;
use crate::auth::session::{AuthState, Session};

#[component]
pub fn AppLayout() -> Element {
    let route = use_route::<Route>();
    let session = use_context::<Session>();

    // 登录态守卫:
    // - Loading: restore 进行中,显示加载态(App 根组件的 use_effect 会完成 restore)
    // - Guest: 未登录,在 effect 里跳转到 login,渲染期先显示加载态
    // - Authenticated: 正常渲染
    match *session.state.read() {
        AuthState::Authenticated { .. } => {}
        AuthState::Guest => {
            use_effect(move || {
                navigator().replace(Route::Login {});
            });
            return rsx! {
                div { class: "splash-screen",
                    div { class: "splash-logo", "…" }
                }
            };
        }
        AuthState::Loading => {
            return rsx! {
                div { class: "splash-screen",
                    div { class: "splash-logo", "…" }
                }
            };
        }
    }

    let active_tab = match route {
        Route::Briefing { .. } => TabId::Briefing,
        Route::AiChat { .. } => TabId::Ai,
        Route::Me { .. } => TabId::Me,
        _ => TabId::Briefing,
    };

    rsx! {
        div { class: "app-container",
            StatusBar {}
            div { class: "screen-body",
                div { class: "page",
                    Outlet::<Route> {}
                }
            }
            TabBar { active_tab }
        }
    }
}
