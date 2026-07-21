use dioxus::prelude::*;

use crate::api;
use crate::api::types::{ApiError, MeResponse, UserInfo};
use crate::auth::token::{self, StoredToken};

/// 全局鉴权状态
#[derive(Clone, PartialEq)]
pub enum AuthState {
    Loading,
    Guest,
    Authenticated {
        access_token: String,
        user: UserInfo,
        refresh_token: String,
        expires_at: i64,
    },
}

/// 通过 Context 注入的全局 Session
#[derive(Clone, Copy)]
pub struct Session {
    pub state: Signal<AuthState>,
}

impl Session {
    pub fn new() -> Self {
        let state = use_signal(|| AuthState::Loading);
        Self { state }
    }

    pub fn set_guest(&mut self) {
        self.state.set(AuthState::Guest);
    }

    pub fn set_authed(&mut self, access: String, refresh: String, expires_in: u32, user: UserInfo) {
        let expires_at = chrono::Utc::now().timestamp() + expires_in as i64 - 60;
        token::save_tokens(&access, &refresh, expires_at);
        self.state.set(AuthState::Authenticated {
            access_token: access,
            user,
            refresh_token: refresh,
            expires_at,
        });
    }

    pub fn logout(&mut self) {
        token::clear_tokens();
        self.state.set(AuthState::Guest);
    }

    pub fn is_authed(&self) -> bool {
        matches!(*self.state.read(), AuthState::Authenticated { .. })
    }

    /// 从登录响应直接建立会话(注册成功后也可调用)
    pub fn apply_login(&mut self, resp: &api::types::LoginResponse) {
        self.set_authed(
            resp.access_token.clone(),
            resp.refresh_token.clone(),
            resp.expires_in,
            resp.user.clone(),
        );
    }
}

/// 启动时的 token 校验流程:读 localStorage -> 过期则 refresh -> me 验证。
/// 返回 Some 表示已登录,None 表示需跳登录页。
pub async fn restore_session() -> Option<RestoreOutcome> {
    let stored = token::load_tokens()?;
    if token::is_expired(&stored) {
        refresh_and_me(&stored.refresh_token).await
    } else {
        match api::me(&stored.access_token).await {
            Ok(user) => Some(RestoreOutcome::Authed {
                access: stored.access_token,
                refresh: stored.refresh_token,
                expires_at: stored.expires_at,
                user,
            }),
            Err(ApiError::Unauthorized) => refresh_and_me(&stored.refresh_token).await,
            Err(_) => {
                token::clear_tokens();
                None
            }
        }
    }
}

pub enum RestoreOutcome {
    Authed {
        access: String,
        refresh: String,
        expires_at: i64,
        user: MeResponse,
    },
}

async fn refresh_and_me(refresh_token: &str) -> Option<RestoreOutcome> {
    let refreshed = api::refresh(refresh_token.to_string()).await.ok()?;
    let new_expires_at = chrono::Utc::now().timestamp() + refreshed.expires_in as i64 - 60;
    token::save_tokens(&refreshed.access_token, refresh_token, new_expires_at);
    let user = api::me(&refreshed.access_token).await.ok()?;
    Some(RestoreOutcome::Authed {
        access: refreshed.access_token,
        refresh: refresh_token.to_string(),
        expires_at: new_expires_at,
        user,
    })
}
