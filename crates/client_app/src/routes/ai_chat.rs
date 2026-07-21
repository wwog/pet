use dioxus::prelude::*;

// ── Tab: AI 宠物翻译官 ──────────────────────────────────────
#[component]
pub fn AiChat() -> Element {
    let mut active_mode = use_signal(|| ModeId::Triage);

    rsx! {
        div { class: "ai-nav-bar",
            div { style: "width: 32px" }
            div { style: "flex: 1; text-align: center",
                div { class: "title", "宠物翻译官" }
                div { class: "sub", "豆豆 · 金毛 · AI 实时解读" }
            }
            div { style: "width: 32px" }
        }

        div { class: "mode-row",
            span {
                class: if active_mode() == ModeId::Triage { "mode-chip active" } else { "mode-chip" },
                onclick: move |_| active_mode.set(ModeId::Triage),
                svg { view_box: "0 0 14 14", fill: "none",
                    path { d: "M7 2l5 9H2L7 2z", stroke: "currentColor", "stroke-width": "1.5",
                        "stroke-linejoin": "round" }
                    path { d: "M7 6v2M7 9.5v.5", stroke: "currentColor", "stroke-width": "1.5",
                        "stroke-linecap": "round" }
                }
                "紧急分诊"
            }
            span {
                class: if active_mode() == ModeId::Behavior { "mode-chip active" } else { "mode-chip" },
                onclick: move |_| active_mode.set(ModeId::Behavior),
                "行为解读"
            }
            span {
                class: if active_mode() == ModeId::Personify { "mode-chip active" } else { "mode-chip" },
                onclick: move |_| active_mode.set(ModeId::Personify),
                "🐾 拟人化"
            }
        }

        div { class: "chat-body",
            // AI message
            div { class: "msg ai",
                div { class: "role",
                    span { class: "ai-av",
                        svg { width: "12", height: "12", view_box: "0 0 12 12",
                            circle { cx: "6", cy: "6", r: "4", fill: "#fff" }
                        }
                    }
                    "宠物翻译官"
                }
                div { class: "bubble",
                    "你好，豆豆的家长。描述一下豆豆现在的症状或行为，我会判断紧急程度并给出建议。也可以上传一段视频让我看看。"
                }
                div { class: "time", "9:38" }
            }

            // User message
            div { class: "msg me",
                div { class: "bubble",
                    "豆豆从半小时前开始反复干呕，吐了两次黄水，精神有点蔫，鼻子发干。"
                }
                div { class: "time", "9:40" }
            }

            // AI response with triage card
            div { class: "msg ai",
                div { class: "role",
                    span { class: "ai-av",
                        svg { width: "12", height: "12", view_box: "0 0 12 12",
                            circle { cx: "6", cy: "6", r: "4", fill: "#fff" }
                        }
                    }
                    "宠物翻译官"
                }
                div { class: "bubble",
                    "综合症状分析，建议尽快就医排查。"
                    div { class: "triage-card",
                        div { class: "triage-head",
                            span { class: "triage-level red", "红色 · 紧急" }
                            span { class: "t", "疑似胃扩张或异物阻塞" }
                        }
                        div { class: "d",
                            "反复干呕 + 黄水 + 精神萎靡是高危信号。就医前：禁食禁水，避免剧烈活动，记录呕吐次数与内容。"
                        }
                        div { class: "triage-actions",
                            button { class: "nav-action nav", "导航至24h急诊" }
                            button { class: "nav-action call", "联系兽医" }
                        }
                    }
                }
                div { class: "time", "9:40 · AI 建议，非诊断" }
            }

            // Typing indicator
            div { class: "msg ai",
                div { class: "role",
                    span { class: "ai-av",
                        svg { width: "12", height: "12", view_box: "0 0 12 12",
                            circle { cx: "6", cy: "6", r: "4", fill: "#fff" }
                        }
                    }
                    "正在为你查找附近急诊医院…"
                }
                div { class: "bubble",
                    div { class: "typing",
                        span {}
                        span {}
                        span {}
                    }
                }
            }
        }

        // Input bar
        div { class: "input-bar",
            div { class: "attach",
                svg { width: "18", height: "18", view_box: "0 0 18 18", fill: "none",
                    path { d: "M9 4v10M4 9h10", stroke: "currentColor", "stroke-width": "1.8",
                        "stroke-linecap": "round" }
                }
            }
            input { "type": "text", placeholder: "描述症状或上传视频…" }
            button { class: "send",
                svg { width: "18", height: "18", view_box: "0 0 18 18", fill: "none",
                    path { d: "M3 9l12-5-4 12-3-5-5-2z", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linejoin": "round" }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ModeId {
    Triage,
    Behavior,
    Personify,
}
