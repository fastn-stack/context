//! # fastn-context
//!
//! Type-safe async context propagation for Rust applications with hierarchical task management
//! and graceful cancellation support.
//!
//! ## Overview
//!
//! `fastn-context` provides a robust context management system designed for async Rust applications.
//! It enables hierarchical context propagation, graceful cancellation, and comprehensive status
//! tracking across your application's task tree.
//!
//! ## Key Features
//!
//! - **Hierarchical Context Management**: Create parent-child relationships between contexts
//! - **Graceful Cancellation**: Built on `tokio::CancellationToken` for clean shutdowns
//! - **Status Tracking**: Monitor the state and progress of operations across your application
//! - **Minimal Overhead**: Lightweight design with efficient async operations
//! - **Easy Integration**: Simple APIs that integrate seamlessly with existing Rust async code
//!
//! ## Quick Start
//!
//! ### Basic Usage
//!
//! ```rust
//! use fastn_context::Context;
//! use tokio::time::{sleep, Duration};
//!
//! #[tokio::main]
//! async fn main() {
//!     let ctx = Context::builder("my-app").build();
//!     
//!     // Spawn a task with context
//!     let child_ctx = ctx.child("worker");
//!     tokio::spawn(async move {
//!         // Your async work here
//!         sleep(Duration::from_millis(100)).await;
//!         println!("Work completed");
//!     });
//!
//!     // Monitor status
//!     let status = fastn_context::status().await;
//!     println!("Active contexts: {}", status.contexts.len());
//! }
//! ```
//!
//! ### Using the `#[main]` Macro
//!
//! For applications that need automatic context setup:
//!
//! ```rust
//! use fastn_context::main;
//!
//! #[main]
//! async fn main() {
//!     // Global context is automatically available
//!     let ctx = fastn_context::global().await;
//!     println!("App: {}", ctx.name());
//! }
//! ```
//!
//! ### Cancellation and Shutdown
//!
//! ```rust
//! use fastn_context::Context;
//! use tokio::select;
//! use tokio::time::{sleep, Duration};
//!
//! #[tokio::main]
//! async fn main() {
//!     let ctx = Context::builder("my-app").build();
//!     
//!     let child_ctx = ctx.child("worker");
//!     let token = child_ctx.cancellation_token();
//!     
//!     tokio::spawn(async move {
//!         select! {
//!             _ = token.cancelled() => {
//!                 println!("Task was cancelled");
//!             }
//!             _ = sleep(Duration::from_secs(10)) => {
//!                 println!("Task completed normally");
//!             }
//!         }
//!     });
//!     
//!     // Cancel after 1 second
//!     sleep(Duration::from_secs(1)).await;
//!     child_ctx.cancel();
//! }
//! ```
//!
//! ## Architecture
//!
//! The crate is built around three main components:
//!
//! - [`Context`]: The core context type for hierarchical task management
//! - [`ContextStatus`]: Status information and monitoring capabilities  
//! - [`Status`]: Global status snapshots of the entire context tree
//!
//! ## Integration with fastn Applications
//!
//! This crate was extracted from the [fastn](https://github.com/fastn-stack/fastn) web framework
//! to enable broader adoption of its context management patterns. It's designed to work
//! seamlessly with fastn applications while being useful for any async Rust project.

#![warn(unused_extern_crates)]
#![deny(unused_crate_dependencies)]

use tokio as _; // used by main macro
use tokio_util as _; // used for cancellation tokens

mod context;
mod status;

pub use context::{Context, ContextBuilder, global};
pub use status::{ContextStatus, Status, status, status_with_latest};

// Re-export main macro
pub use fastn_context_macros::main;
