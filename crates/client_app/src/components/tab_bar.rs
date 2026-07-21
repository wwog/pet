use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TabId {
    Briefing,
    Ai,
    Me,
}

impl TabId {
    pub fn indicator_left(&self) -> &'static str {
        match self {
            TabId::Briefing => "calc(16.667% - 14px)",
            TabId::Ai => "calc(50% - 14px)",
            TabId::Me => "calc(83.333% - 14px)",
        }
    }
}

#[component]
pub fn TabBar(active_tab: TabId) -> Element {
    rsx! {
        nav { class: "tab-bar",
            div { class: "tab-indicator", left: active_tab.indicator_left() }

            Link {
                class: if active_tab == TabId::Briefing { "tab-item active" } else { "tab-item" },
                to: crate::app::Route::Briefing {},
                svg { view_box: "0 0 26 26", fill: "currentColor",
                    path { d: "M13 3l9 7v10a2 2 0 0 1-2 2h-4v-7h-6v7H6a2 2 0 0 1-2-2V10l9-7z" }
                }
                span { class: "tab-label", "简报" }
            }

            Link {
                class: if active_tab == TabId::Ai { "tab-item active" } else { "tab-item" },
                to: crate::app::Route::AiChat {},
                svg {
                    view_box: "0 0 26 26",
                    fill: if active_tab == TabId::Ai { "currentColor" } else { "none" },
                    stroke: "currentColor",
                    "stroke-width": "1.8",
                    path { d: "M5 6h16a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H9l-5 4V8a2 2 0 0 1 1-2z" }
                }
                span { class: "tab-label", "AI" }
            }

            Link {
                class: if active_tab == TabId::Me { "tab-item active" } else { "tab-item" },
                to: crate::app::Route::Me {},
                svg {
                    view_box: "0 0 26 26",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "1.8",
                    circle { cx: "13", cy: "9", r: "4" }
                    path { d: "M5 21c0-4 3.5-6 8-6s8 2 8 6" }
                }
                span { class: "tab-label", "我的" }
            }
        }
    }
}
