use dioxus::prelude::*;

use crate::components::pet_header::PetHeader;

#[component]
pub fn Briefing() -> Element {
    rsx! {
        PetHeader {}

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
