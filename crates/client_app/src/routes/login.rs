use dioxus::prelude::*;

use crate::api;
use crate::auth::session::Session;
use crate::auth::validate::validate_account;

#[component]
pub fn Login() -> Element {
    let mut account = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut show_pw = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut loading = use_signal(|| false);

    let do_login = move |_| async move {
        let acct = account.read().clone();
        let pw = password.read().clone();

        if let Err(e) = validate_account(&acct) {
            error.set(e);
            return;
        }
        if pw.len() < 8 {
            error.set("密码至少8位".into());
            return;
        }

        loading.set(true);
        error.set(String::new());

        match api::login(acct, pw).await {
            Ok(resp) => {
                let mut session = use_context::<Session>();
                session.apply_login(&resp);
                let nav = navigator();
                nav.replace(crate::app::Route::Briefing {});
            }
            Err(api::types::ApiError::Server(code, msg)) => {
                error.set(format!("[{code}] {msg}"));
            }
            Err(e) => {
                error.set(format!("登录失败: {e}"));
            }
        }
        loading.set(false);
    };

    rsx! {
        div { class: "auth-screen",
            div { class: "auth-logo",
                svg { width: "64", height: "64", view_box: "0 0 26 26",
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
            h1 { class: "auth-title", "小狗人生" }
            p { class: "auth-subtitle", "Puppy Life OS" }
            p { class: "auth-slogan", "用科技的温度,延伸爱的刻度" }

            div { class: "auth-form",
                div { class: "auth-field",
                    label { "账号" }
                    input {
                        r#type: "text",
                        placeholder: "字母数字下划线,至少6位",
                        value: "{account}",
                        oninput: move |e| account.set(e.value()),
                    }
                }
                div { class: "auth-field",
                    label { "密码" }
                    div { class: "pw-wrap",
                        input {
                            r#type: if show_pw() { "text" } else { "password" },
                            placeholder: "至少8位,含字母和数字",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                        button {
                            class: "pw-toggle",
                            r#type: "button",
                            onclick: move |_| show_pw.set(!show_pw()),
                            if show_pw() { "🙈" } else { "👁" }
                        }
                    }
                }

                if !error.read().is_empty() {
                    div { class: "auth-error", "{error}" }
                }

                button {
                    class: "auth-submit",
                    disabled: *loading.read(),
                    onclick: do_login,
                    if *loading.read() { "登录中…" } else { "登录" }
                }

                div { class: "auth-switch",
                    span { "还没有账号?" }
                    Link { to: crate::app::Route::Register {}, "注册账号" }
                }
            }
        }
    }
}
