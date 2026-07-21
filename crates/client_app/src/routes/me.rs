use dioxus::prelude::*;

use crate::app::Route;
use crate::auth::session::Session;

// ── Tab: 我的 ────────────────────────────────────────────────
#[component]
pub fn Me() -> Element {
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

        // 退出登录
        div { class: "section-label", "账号" }
        button {
            class: "logout-btn",
            onclick: move |_| {
                let mut session = use_context::<Session>();
                session.logout();
                let nav = navigator();
                nav.replace(Route::Login {});
            },
            "退出登录"
        }
    }
}
