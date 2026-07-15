//! 领域层 — 核心业务模型与 Repository 契约。
//!
//! 本 crate 只依赖 serde、uuid、chrono、thiserror、async-trait，
//! 不依赖任何基础设施或框架。所有外部交互通过 repository trait 抽象。

pub mod app;
pub mod user;
pub mod family;
pub mod pet;