use dioxus::prelude::*;

use crate::api;
use crate::api::types::ApiError;
use crate::auth::session::Session;
use crate::auth::validate::{validate_account, validate_confirm, validate_nickname, validate_password};

#[component]
pub fn Register() -> Element {
    let mut account = use_signal(String::new);
    let mut nickname = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut show_pw = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut loading = use_signal(|| false);

    let do_register = move |_| async move {
        let acct = account.read().clone();
        let nick = nickname.read().clone();
        let pw = password.read().clone();
        let cf = confirm.read().clone();

        if let Err(e) = validate_account(&acct) {
            error.set(e); return;
        }
        if let Err(e) = validate_nickname(&nick) {
            error.set(e); return;
        }
        if let Err(e) = validate_password(&pw) {
            error.set(e); return;
        }
        if let Err(e) = validate_confirm(&pw, &cf) {
            error.set(e); return;
        }

        loading.set(true);
        error.set(String::new());

        // 注册 -> 自动登录(后端 register 不返回 token,需再调 login)
        match api::register(acct.clone(), pw.clone(), nick.clone()).await {
            Ok(_) => {
                match api::login(acct, pw).await {
                    Ok(login_resp) => {
                        let mut session = use_context::<Session>();
                        session.apply_login(&login_resp);
                        let nav = navigator();
                        nav.replace(crate::app::Route::Briefing {});
                    }
                    Err(e) => {
                        error.set(format!("注册成功但自动登录失败: {e}"));
                    }
                }
            }
            Err(ApiError::Server(code, msg)) => {
                error.set(format!("[{code}] {msg}"));
            }
            Err(e) => {
                error.set(format!("注册失败: {e}"));
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
            h1 { class: "auth-title", "创建账号" }
            p { class: "auth-slogan", "给毛孩子一个数字档案" }

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
                    label { "昵称" }
                    input {
                        r#type: "text",
                        placeholder: "1-20 字符",
                        value: "{nickname}",
                        oninput: move |e| nickname.set(e.value()),
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
                div { class: "auth-field",
                    label { "确认密码" }
                    input {
                        r#type: if show_pw() { "text" } else { "password" },
                        placeholder: "再次输入密码",
                        value: "{confirm}",
                        oninput: move |e| confirm.set(e.value()),
                    }
                }

                if !error.read().is_empty() {
                    div { class: "auth-error", "{error}" }
                }

                button {
                    class: "auth-submit",
                    disabled: *loading.read(),
                    onclick: do_register,
                    if *loading.read() { "注册中…" } else { "注册并登录" }
                }

                div { class: "auth-switch",
                    span { "已有账号?" }
                    Link { to: crate::app::Route::Login {}, "去登录" }
                }
            }
        }
    }
}
