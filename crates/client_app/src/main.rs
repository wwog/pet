mod api;
mod auth;

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Briefing {},
    #[route("/ai")]
    AiChat {},
    #[route("/me")]
    Me {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[derive(Clone, Copy, PartialEq)]
enum TabId {
    Briefing,
    Ai,
    Me,
}

// ── Layout ──────────────────────────────────────────────────
#[component]
fn AppLayout() -> Element {
    let route = use_route::<Route>();

    let active_tab = match route {
        Route::Briefing { .. } => TabId::Briefing,
        Route::AiChat { .. } => TabId::Ai,
        Route::Me { .. } => TabId::Me,
    };

    let indicator_left = match active_tab {
        TabId::Briefing => "calc(16.667% - 14px)",
        TabId::Ai => "calc(50% - 14px)",
        TabId::Me => "calc(83.333% - 14px)",
    };

    rsx! {
        div {
            class: "app-container",

            div { class: "status-bar",
                span { "9:41" }
                span { class: "right",
                    svg { width: "17", height: "11", view_box: "0 0 17 11", fill: "currentColor",
                        rect { x: "0", y: "6", width: "3", height: "5", rx: "1" }
                        rect { x: "4", y: "4", width: "3", height: "7", rx: "1" }
                        rect { x: "8", y: "2", width: "3", height: "9", rx: "1" }
                        rect { x: "12", y: "0", width: "3", height: "11", rx: "1" }
                    }
                    svg { width: "16", height: "11", view_box: "0 0 16 11", fill: "currentColor",
                        path { d: "M8 2.5c2 0 3.8.8 5.2 2L14 3.4C12.4 1.8 10.3 1 8 1S3.6 1.8 2 3.4l.8 1.1C4.2 3.3 6 2.5 8 2.5z" }
                        path { d: "M8 5.5c1.2 0 2.3.5 3.1 1.3l.8-1.1C11 4.6 9.5 4 8 4s-3 .6-3.9 1.7l.8 1.1C5.7 6 6.8 5.5 8 5.5z" }
                        circle { cx: "8", cy: "9", r: "1.5" }
                    }
                    svg { width: "27", height: "13", view_box: "0 0 27 13", fill: "none",
                        rect { x: "0.5", y: "0.5", width: "22", height: "12", rx: "3.5", stroke: "currentColor", opacity: "0.4" }
                        rect { x: "2", y: "2", width: "19", height: "9", rx: "2", fill: "currentColor" }
                        rect { x: "24", y: "4", width: "2", height: "5", rx: "1", fill: "currentColor", opacity: "0.4" }
                    }
                }
            }

            div { class: "screen-body",
                div { class: "page",
                    Outlet::<Route> {}
                }
            }

            nav {
                class: "tab-bar",

                div {
                    class: "tab-indicator",
                    left: indicator_left,
                }

                Link {
                    class: if active_tab == TabId::Briefing { "tab-item active" } else { "tab-item" },
                    to: Route::Briefing {},
                    svg {
                        view_box: "0 0 26 26",
                        fill: "currentColor",
                        path { d: "M13 3l9 7v10a2 2 0 0 1-2 2h-4v-7h-6v7H6a2 2 0 0 1-2-2V10l9-7z" }
                    }
                    span { class: "tab-label", "简报" }
                }

                Link {
                    class: if active_tab == TabId::Ai { "tab-item active" } else { "tab-item" },
                    to: Route::AiChat {},
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
                    to: Route::Me {},
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
}

// ── Tab: 简报 ────────────────────────────────────────────────
#[component]
fn Briefing() -> Element {
    rsx! {
        div { class: "top-row",
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

        div { class: "pet-switch",
            div { class: "pet-av",
                svg { width: "24", height: "24", view_box: "0 0 26 26",
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

        // Walk CTA
        a { class: "walk-cta",
            span { class: "walk-cta-ic",
                svg { view_box: "0 0 22 22", fill: "none",
                    circle { cx: "13", cy: "5", r: "2.2", stroke: "currentColor", "stroke-width": "1.6" }
                    path { d: "M11 9l-3.5 3 2 2 2-2M10 12l-2 5.5M13 12l3.5-1M15 16.5l-1-3.5",
                        stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
            span { class: "walk-cta-txt",
                span { class: "n", "开始遛狗" }
                span { class: "d", "记录轨迹 · 速度 · 事件打卡" }
            }
            span { class: "walk-cta-arrow",
                svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "none",
                    path { d: "M6 3l5 5-5 5", stroke: "currentColor", "stroke-width": "2",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }

        // AI 简报 Hero
        div { class: "briefing-hero",
            div { class: "date-row",
                span { "7月7日 · 周二" }
                span { class: "weather",
                    svg { width: "12", height: "12", view_box: "0 0 12 12", fill: "currentColor",
                        circle { cx: "6", cy: "6", r: "3" }
                        path { d: "M6 1v1.5M6 9.5V11M1 6h1.5M9.5 6H11M2.5 2.5l1 1M8.5 8.5l1 1M9.5 2.5l-1 1M3.5 8.5l-1 1",
                            stroke: "currentColor", "stroke-width": "1", "stroke-linecap": "round" }
                    }
                    " 28° 多云"
                }
            }
            h1 { "今天适合多陪豆豆散步" }
            div { class: "quote",
                "傍晚 18:00 气温回落到 24°，是金毛一天里最舒服的遛弯时段。它最近学会了\"握手\"，别忘了奖励它。"
            }
            div { class: "ai-tag", "AI · DAILY BRIEFING" }
        }

        // Walk card
        div { class: "walk-card",
            div { class: "walk-ring",
                div { class: "inner",
                    div {
                        span { class: "min", "41" }
                        span { class: "unit", "min" }
                    }
                }
            }
            div { class: "walk-info",
                div { class: "label", "今日遛弯" }
                div { class: "val", "目标 60 分钟" }
                div { class: "sub", "还差 19 分钟 · 奶爸 阿哲 负责晚遛" }
            }
        }

        // Stats grid
        div { class: "grid-2",
            div { class: "mini-stat",
                div { class: "top",
                    span { class: "label", "昨日睡眠" }
                    svg { class: "icon", width: "16", height: "16", view_box: "0 0 16 16", fill: "none",
                        path { d: "M13 9a5 5 0 1 1-6-6 6 6 0 0 0 6 6z", stroke: "currentColor", "stroke-width": "1.3" }
                    }
                }
                div { class: "val", "11.2", span { class: "u", "小时" } }
                div { class: "delta up", "↑ 比平均多 1.4h · 深睡充足" }
            }
            div { class: "mini-stat",
                div { class: "top",
                    span { class: "label", "情绪指数" }
                    svg { class: "icon", width: "16", height: "16", view_box: "0 0 16 16", fill: "none",
                        circle { cx: "8", cy: "8", r: "6", stroke: "currentColor", "stroke-width": "1.3" }
                        circle { cx: "6", cy: "7", r: "0.8", fill: "currentColor" }
                        circle { cx: "10", cy: "7", r: "0.8", fill: "currentColor" }
                        path { d: "M5.5 10.5c1 1 4 1 5 0", stroke: "currentColor", "stroke-width": "1.3",
                            "stroke-linecap": "round" }
                    }
                }
                div { class: "val", "92", span { class: "u", "/100" } }
                div { class: "delta up", "↑ 开心 · 活跃" }
            }
        }

        // Todo
        div { class: "section-label", "今日待办 · 按优先级" }
        div { class: "todo-card",
            div { class: "h",
                span { class: "t", "3 件待办" }
                span { class: "ct", "奶妈协助" }
            }
            div { class: "todo-item",
                div { class: "todo-pri high" }
                div { class: "todo-text",
                    "体外驱虫（福来恩）"
                    div { class: "who", "奶爸 阿哲 · 已逾期 2 天" }
                }
                div { class: "todo-time", "今日" }
            }
            div { class: "todo-item",
                div { class: "todo-pri mid" }
                div { class: "todo-text",
                    "晚餐喂粮 320g"
                    div { class: "who", "奶妈 小棠" }
                }
                div { class: "todo-time", "18:30" }
            }
            div { class: "todo-item",
                div { class: "todo-pri low" }
                div { class: "todo-text",
                    "刷牙训练 5 分钟"
                    div { class: "who", "爷爷 老张" }
                }
                div { class: "todo-time", "21:00" }
            }
        }
    }
}

// ── Tab: AI 宠物翻译官 ──────────────────────────────────────
#[component]
fn AiChat() -> Element {
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

// ── Tab: 我的 ────────────────────────────────────────────────
#[component]
fn Me() -> Element {
    rsx! {
        div { class: "me-head",
            div { class: "av", "哲" }
            div { class: "info",
                div { class: "name", "阿哲" }
                div { class: "fam",
                    "阿哲的家"
                    span { class: "chip-role", "首席监护人" }
                }
            }
        }

        // Pet card
        a { class: "pet-card",
            div { class: "pav",
                svg { width: "28", height: "28", view_box: "0 0 28 28",
                    ellipse { cx: "14", cy: "18", rx: "10", ry: "7", fill: "#fff" }
                    circle { cx: "14", cy: "12", r: "8", fill: "#fff" }
                    ellipse { cx: "8", cy: "9", rx: "3", ry: "5", fill: "#fff",
                        transform: "rotate(-20 8 9)" }
                    ellipse { cx: "20", cy: "9", rx: "3", ry: "5", fill: "#fff",
                        transform: "rotate(20 20 9)" }
                    circle { cx: "11", cy: "12", r: "1", fill: "#3a2a1a" }
                    circle { cx: "17", cy: "12", r: "1", fill: "#3a2a1a" }
                }
            }
            div { class: "info",
                div { class: "n", "豆豆 · 金毛" }
                div { class: "m", "1岁2个月 · 已陪伴 287 天" }
            }
            span { class: "add-pet", "+ 加宠物" }
        }

        // Daily functions grid
        div { class: "section-label", "养宠日常" }
        div { class: "grid-2",
            a { class: "entry",
                div { class: "ic mint",
                    svg { view_box: "0 0 18 18", fill: "none",
                        rect { x: "2.5", y: "4", width: "13", height: "11", rx: "2",
                            stroke: "currentColor", "stroke-width": "1.5" }
                        circle { cx: "6.5", cy: "8", r: "1.5", stroke: "currentColor", "stroke-width": "1.5" }
                        path { d: "M2.5 12l3.5-2.5 2.5 2 3-2.5 3.5 2", stroke: "currentColor",
                            "stroke-width": "1.5" }
                    }
                }
                div { class: "t", "云相册" }
                div { class: "d", "1,284 张 · AI 标签检索" }
            }
            a { class: "entry",
                div { class: "ic accent",
                    svg { view_box: "0 0 18 18", fill: "none",
                        rect { x: "2.5", y: "4", width: "13", height: "12", rx: "2",
                            stroke: "currentColor", "stroke-width": "1.5" }
                        path { d: "M2.5 7.5h13M6 2.5v3M12 2.5v3", stroke: "currentColor",
                            "stroke-width": "1.5", "stroke-linecap": "round" }
                    }
                }
                div { class: "t", "智能日程" }
                div { class: "d", "3 待办 · 1 逾期" }
            }
            a { class: "entry",
                div { class: "ic tan",
                    svg { view_box: "0 0 18 18", fill: "none",
                        path { d: "M9 15s-5-3-5-7a2.8 2.8 0 0 1 5-1.5A2.8 2.8 0 0 1 14 8c0 4-5 7-5 7z",
                            stroke: "currentColor", "stroke-width": "1.5", "stroke-linejoin": "round" }
                    }
                }
                div { class: "t", "健康档案" }
                div { class: "d", "过敏 2 项 · 驱虫逾期" }
            }
            a { class: "entry",
                div { class: "ic mint",
                    svg { view_box: "0 0 18 18", fill: "none",
                        circle { cx: "10", cy: "4", r: "1.8", stroke: "currentColor", "stroke-width": "1.5" }
                        path { d: "M8.5 7.5l-2.5 2 1.5 1.5 1.5-1.5M7.5 10l-1.5 4M10 9.5l2.5-1M11.5 12.5l-1-2.5",
                            stroke: "currentColor", "stroke-width": "1.5",
                            "stroke-linecap": "round", "stroke-linejoin": "round" }
                    }
                }
                div { class: "t", "遛狗记录" }
                div { class: "d", "今日 2.34km · 8 事件" }
            }
        }

        // Family & collaboration
        div { class: "section-label", "家庭与协作" }
        a { class: "list-entry",
            div { class: "ic",
                svg { view_box: "0 0 18 18", fill: "none",
                    circle { cx: "9", cy: "6", r: "3", stroke: "currentColor", "stroke-width": "1.5" }
                    path { d: "M3 15c0-3 2.5-4.5 6-4.5s6 1.5 6 4.5", stroke: "currentColor",
                        "stroke-width": "1.5" }
                }
            }
            div { class: "info",
                div { class: "n", "家庭与权限" }
                div { class: "d", "3 成员 · 柔性 RBAC" }
            }
            span { class: "badge", "1" }
            span { class: "arrow",
                svg { width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                    path { d: "M5 3l4 4-4 4", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }
        a { class: "list-entry",
            div { class: "ic",
                svg { view_box: "0 0 18 18", fill: "none",
                    path { d: "M9 2v10M5 6l4-4 4 4", stroke: "currentColor", "stroke-width": "1.5",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                    path { d: "M3 13v2a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1v-2", stroke: "currentColor",
                        "stroke-width": "1.5" }
                }
            }
            div { class: "info",
                div { class: "n", "邀请家人" }
                div { class: "d", "6 位邀请码 · 待审核 1 人" }
            }
            span { class: "arrow",
                svg { width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                    path { d: "M5 3l4 4-4 4", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }

        // Advanced tools
        div { class: "section-label", "进阶工具" }
        a { class: "list-entry",
            div { class: "ic",
                svg { view_box: "0 0 18 18", fill: "none",
                    rect { x: "2.5", y: "4", width: "13", height: "10", rx: "2",
                        stroke: "currentColor", "stroke-width": "1.5" }
                    path { d: "M2.5 8h13M5 11h3", stroke: "currentColor",
                        "stroke-width": "1.5", "stroke-linecap": "round" }
                }
            }
            div { class: "info",
                div { class: "n", "财务管家" }
                div { class: "d", "本月 ¥1,240 · 医疗占比 52%" }
            }
            span { class: "arrow",
                svg { width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                    path { d: "M5 3l4 4-4 4", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }
        a { class: "list-entry",
            div { class: "ic",
                svg { view_box: "0 0 18 18", fill: "none",
                    path { d: "M5 6c0-2 2-3 4-3s4 1 4 3v2H5V6z", stroke: "currentColor",
                        "stroke-width": "1.5", "stroke-linejoin": "round" }
                    path { d: "M5 11c0 2 2 3 4 3s4-1 4-3", stroke: "currentColor",
                        "stroke-width": "1.5" }
                }
            }
            div { class: "info",
                div { class: "n", "饮食计算器" }
                div { class: "d", "今日 320g · 热量达标" }
            }
            span { class: "arrow",
                svg { width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                    path { d: "M5 3l4 4-4 4", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }
        a { class: "list-entry",
            div { class: "ic",
                svg { view_box: "0 0 18 18", fill: "none",
                    circle { cx: "9", cy: "9", r: "6", stroke: "currentColor", "stroke-width": "1.5" }
                    circle { cx: "9", cy: "9", r: "3", stroke: "currentColor",
                        "stroke-width": "1.5", opacity: "0.5" }
                    path { d: "M9 9l4-2", stroke: "currentColor", "stroke-width": "1.5",
                        "stroke-linecap": "round" }
                }
            }
            div { class: "info",
                div { class: "n", "遛狗社群雷达" }
                div { class: "d", "附近 5 只同品种" }
            }
            span { class: "arrow",
                svg { width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                    path { d: "M5 3l4 4-4 4", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }
        a { class: "list-entry",
            div { class: "ic",
                svg { view_box: "0 0 18 18", fill: "none",
                    rect { x: "3", y: "7", width: "12", height: "9", rx: "1.5",
                        stroke: "currentColor", "stroke-width": "1.5" }
                    path { d: "M5.5 7V5a3.5 3.5 0 0 1 7 0v2", stroke: "currentColor",
                        "stroke-width": "1.5" }
                    circle { cx: "9", cy: "11.5", r: "1.2", fill: "currentColor" }
                }
            }
            div { class: "info",
                div { class: "n", "文件保险箱" }
                div { class: "d", "血统证书 · 芯片号 · 保险单" }
            }
            span { class: "arrow",
                svg { width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                    path { d: "M5 3l4 4-4 4", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }
        a { class: "list-entry",
            div { class: "ic",
                svg { view_box: "0 0 18 18", fill: "none",
                    circle { cx: "6", cy: "8", r: "2.5", stroke: "currentColor", "stroke-width": "1.5" }
                    path { d: "M2 15c0-2.5 1.8-4 4-4s4 1.5 4 4", stroke: "currentColor",
                        "stroke-width": "1.5" }
                    path { d: "M11 5l3 3M14 5v3", stroke: "currentColor", "stroke-width": "1.5",
                        "stroke-linecap": "round" }
                }
            }
            div { class: "info",
                div { class: "n", "访客临时模式" }
                div { class: "d", "生成 24h 只读链接" }
            }
            span { class: "arrow",
                svg { width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                    path { d: "M5 3l4 4-4 4", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }

        // Settings
        div { class: "section-label", "设置与隐私" }
        a { class: "list-entry",
            div { class: "ic",
                svg { view_box: "0 0 18 18", fill: "none",
                    path { d: "M9 2v8M5 7l4 3 4-3", stroke: "currentColor", "stroke-width": "1.5",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                    path { d: "M3 13v2a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1v-2", stroke: "currentColor",
                        "stroke-width": "1.5" }
                }
            }
            div { class: "info",
                div { class: "n", "数据导出" }
                div { class: "d", "PDF / CSV · 全档案备份" }
            }
            span { class: "arrow",
                svg { width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                    path { d: "M5 3l4 4-4 4", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }
        a { class: "list-entry",
            div { class: "ic",
                svg { view_box: "0 0 18 18", fill: "none",
                    circle { cx: "9", cy: "9", r: "6", stroke: "currentColor", "stroke-width": "1.5" }
                    path { d: "M9 7v3M9 12v.5", stroke: "currentColor", "stroke-width": "1.5",
                        "stroke-linecap": "round" }
                }
            }
            div { class: "info",
                div { class: "n", "关于小狗人生" }
                div { class: "d", "用科技的温度，延伸爱的刻度" }
            }
            span { class: "arrow",
                svg { width: "14", height: "14", view_box: "0 0 14 14", fill: "none",
                    path { d: "M5 3l4 4-4 4", stroke: "currentColor", "stroke-width": "1.6",
                        "stroke-linecap": "round", "stroke-linejoin": "round" }
                }
            }
        }
    }
}