use dioxus::prelude::*;

use super::status_bar::StatusBar;
use super::tab_bar::{TabBar, TabId};
use crate::app::Route;

#[component]
pub fn AppLayout() -> Element {
    let route = use_route::<Route>();

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
