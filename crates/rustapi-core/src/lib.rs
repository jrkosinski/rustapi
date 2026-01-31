//! RustAPI Core
//!
//! Core runtime for the rustapi framework, providing:
//! - Dependency injection container
//! - Application builder
//! - Service lifecycle management
//! - HTTP server runtime
//! - Re-exported routing and middleware types

pub mod di;
pub mod app;
pub mod error;
pub mod server;

pub use di::{Container, Injectable};
pub use app::App;
pub use error::{Error, Result};
pub use server::RustAPI;

// Re-export axum types so users don't need to depend on axum directly
pub use axum::Router;
pub mod routing {
    pub use axum::routing::*;
}

// Re-export common middleware
pub use tower_http::cors::CorsLayer;
pub use tower_http::trace::TraceLayer;
