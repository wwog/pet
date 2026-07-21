use dioxus::prelude::*;

use crate::app::Route;
use crate::auth::session::{AuthState, Session};

/// 启动页:展示加载动画,监听 Session 状态变化后跳转。
/// 实际的 token restore 在 App 根组件的 use_effect 里执行,
/// 这样无论用户从哪个 URL 进入都会先 restore(避免绕过 Splash)。
#[component]
pub fn Splash() -> Element {
    let session = use_context::<Session>();
    let state = session.state;

    use_effect(move || {
        let nav = navigator();
        match *state.read() {
            AuthState::Authenticated { .. } => {
                nav.replace(Route::Briefing {});
            }
            AuthState::Guest => {
                nav.replace(Route::Login {});
            }
            AuthState::Loading => {}
        }
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
