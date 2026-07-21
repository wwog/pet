use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

/// Briefing 顶部宠物头部:插画背景 + 家庭切换 + 宠物信息。
/// 替代原 main.rs 的 top-row + pet-switch,宠物头像上移到距顶 ~70px。
#[component]
pub fn PetHeader() -> Element {
    rsx! {
        div { class: "pet-header",
            img { class: "bg", src: HEADER_SVG, alt: "" }

            div { class: "family-row",
                div { class: "family-switch",
                    span { class: "fname", "阿哲的家" }
                    svg { class: "chev", width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                        path { d: "M4 6l3 3 3-3", stroke: "currentColor", "stroke-width": "1.8",
                            "stroke-linecap": "round", "stroke-linejoin": "round" }
                    }
                }
                div { class: "members-avatars",
                    div { class: "av", style: "background: var(--accent)" }
                    div { class: "av", style: "background: var(--mint)" }
                    div { class: "av", style: "background: var(--illu-tan)" }
                }
            }

            div { class: "pet-row",
                div { class: "pet-av",
                    svg { width: "28", height: "28", view_box: "0 0 26 26",
                        circle { cx: "13", cy: "15", r: "8", fill: "#fff" }
                        circle { cx: "13", cy: "11", r: "6", fill: "#fff" }
                        ellipse { cx: "8", cy: "8", rx: "2.5", ry: "4", fill: "#fff",
                            transform: "rotate(-20 8 8)" }
                        ellipse { cx: "18", cy: "8", rx: "2.5", ry: "4", fill: "#fff",
                            transform: "rotate(20 18 8)" }
                        circle { cx: "11", cy: "11", r: "1", fill: "#3a2a1a" }
                        circle { cx: "15", cy: "11", r: "1", fill: "#3a2a1a" }
                    }
                }
                div { class: "pet-info",
                    div { class: "pn", "豆豆" }
                    div { class: "pm", "金毛 · 1岁2个月 · 男孩" }
                }
                div { class: "pet-tabs",
                    div { class: "pet-tab", style: "background:var(--accent);color:#fff", "豆" }
                    div { class: "pet-tab", "+" }
                }
            }
        }
    }
}
