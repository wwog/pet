use dioxus::prelude::*;

#[component]
pub fn StatusBar() -> Element {
    rsx! {
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
    }
}
