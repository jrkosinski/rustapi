//! Rusty-Resty: FastAPI-inspired REST framework for Rust
//!
//! Rusty-Resty brings the developer experience of FastAPI and NestJS to Rust,
//! with automatic OpenAPI generation, built-in validation, and dependency injection.
//!
//! # Features
//!
//! - **Route Macros**: Define endpoints with `#[get]`, `#[post]`, etc.
//! - **Dependency Injection**: Type-safe DI container for services
//! - **Type-Driven**: Leverage Rust's type system for validation and docs
//! - **Zero-Cost**: Built on Axum and Tokio for production performance
//!
//! # Quick Start
//!
//! ```ignore
//! use rusty_resty::prelude::*;
//!
//! #[get("/users/{id}")]
//! async fn get_user(Path(id): Path<String>) -> Json<User> {
//!     // handler code
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let app = Router::new()
//!         .route(__get_user_route.0, __get_user_route.1());
//!
//!     axum::serve(listener, app).await.unwrap();
//! }
//! ```
//!
//! # Architecture
//!
//! The framework is organized into several crates:
//!
//! - `rusty-resty-core`: DI container and app builder
//! - `rusty-resty-macros`: Procedural macros for routes
//! - `rusty-resty` (this crate): Facade that re-exports everything
//!
//! # Examples
//!
//! See the `examples/` directory for complete working examples:
//!
//! - `with_macros.rs`: Route macro usage
//! - More coming soon!

//re-export core functionality
pub use rusty_resty_core::{
    Container,
    Injectable,
    App,
    Error as CoreError,
    Result as CoreResult,
};

//re-export macros
pub use rusty_resty_macros::{
    get,
    post,
    put,
    delete,
    patch,
};

//re-export commonly used axum types
pub use axum::{
    Router,
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

//re-export serde for user convenience
pub use serde::{Serialize, Deserialize};

/// Prelude module for convenient imports
///
/// Import everything you need with:
/// ```ignore
/// use rusty_resty::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        //core
        Container,
        Injectable,
        App,
        CoreError,
        CoreResult,

        //macros
        get,
        post,
        put,
        delete,
        patch,

        //axum
        Router,
        Json,
        Path,
        Query,
        State,
        StatusCode,
        IntoResponse,
        Response,

        //serde
        Serialize,
        Deserialize,
    };

    //also re-export tokio for async runtime
    pub use tokio;
}
