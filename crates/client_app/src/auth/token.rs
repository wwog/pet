use crate::api::types::LoginResponse;

const KEY_ACCESS: &str = "petos.access_token";
const KEY_REFRESH: &str = "petos.refresh_token";
const KEY_EXPIRES: &str = "petos.expires_at";

/// 提前 60 秒视为过期,避免边界竞态
const EXPIRY_MARGIN_SECS: i64 = 60;

#[derive(Debug, Clone)]
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

/// 从 LoginResponse 保存 token(expires_in 秒)
pub fn save_from_login(resp: &LoginResponse) {
    let expires_at = now_secs() + resp.expires_in as i64 - EXPIRY_MARGIN_SECS;
    save_tokens(&resp.access_token, &resp.refresh_token, expires_at);
}

pub fn save_tokens(access: &str, refresh: &str, expires_at: i64) {
    ls_set(KEY_ACCESS, access);
    ls_set(KEY_REFRESH, refresh);
    ls_set(KEY_EXPIRES, &expires_at.to_string());
}

pub fn load_tokens() -> Option<StoredToken> {
    let access = ls_get(KEY_ACCESS)?;
    let refresh = ls_get(KEY_REFRESH)?;
    let expires_str = ls_get(KEY_EXPIRES)?;
    let expires_at: i64 = expires_str.parse().ok()?;
    Some(StoredToken {
        access_token: access,
        refresh_token: refresh,
        expires_at,
    })
}

pub fn clear_tokens() {
    ls_remove(KEY_ACCESS);
    ls_remove(KEY_REFRESH);
    ls_remove(KEY_EXPIRES);
}

pub fn is_expired(t: &StoredToken) -> bool {
    now_secs() >= t.expires_at
}

fn now_secs() -> i64 {
    chrono::Utc::now().timestamp()
}

// ── 平台相关 localStorage 访问 ────────────────────────
#[cfg(target_arch = "wasm32")]
fn ls_set(key: &str, value: &str) {
    let Some(win) = web_sys::window() else { return };
    if let Ok(Some(storage)) = win.local_storage() {
        let _ = storage.set_item(key, value);
    }
}

#[cfg(target_arch = "wasm32")]
fn ls_get(key: &str) -> Option<String> {
    let win = web_sys::window()?;
    let storage = win.local_storage().ok()??;
    storage.get_item(key).ok().flatten()
}

#[cfg(target_arch = "wasm32")]
fn ls_remove(key: &str) {
    let Some(win) = web_sys::window() else { return };
    if let Ok(Some(storage)) = win.local_storage() {
        let _ = storage.remove_item(key);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn ls_set(_key: &str, _value: &str) {}

#[cfg(not(target_arch = "wasm32"))]
fn ls_get(_key: &str) -> Option<String> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn ls_remove(_key: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_expired_true_when_past_expiry() {
        let t = StoredToken {
            access_token: "a".into(),
            refresh_token: "r".into(),
            expires_at: now_secs() - 100,
        };
        assert!(is_expired(&t));
    }

    #[test]
    fn is_expired_false_when_before_expiry() {
        let t = StoredToken {
            access_token: "a".into(),
            refresh_token: "r".into(),
            expires_at: now_secs() + 10000,
        };
        assert!(!is_expired(&t));
    }
}
