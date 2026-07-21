/// 把相对路径(如 "/app/auth/login")拼成绝对 URL。
/// wasm 下用 window.location.origin;desktop 下用硬编码后端地址。
#[cfg(target_arch = "wasm32")]
pub fn abs_url(path: &str) -> String {
    let origin = web_sys::window()
        .map(|w| w.location().origin().ok())
        .flatten()
        .unwrap_or_default();
    format!("{origin}{path}")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn abs_url(path: &str) -> String {
    format!("http://127.0.0.1:3000{path}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn abs_url_desktop_prepends_backend() {
        assert_eq!(abs_url("/app/auth/login"), "http://127.0.0.1:3000/app/auth/login");
        assert_eq!(abs_url("/common/auth/me"), "http://127.0.0.1:3000/common/auth/me");
    }
}
