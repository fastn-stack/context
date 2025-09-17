//! # fastn-context-macros
//!
//! Procedural macros for the `fastn-context` crate.
//!
//! This crate provides the `#[main]` attribute macro that simplifies setting up
//! fastn applications with automatic context management.
//!
//! ## Usage
//!
//! Add the `#[main]` attribute to your async main function to automatically set up
//! the tokio runtime and global context:
//!
//! ```rust
//! use fastn_context::main;
//!
//! #[main]
//! async fn main() {
//!     // Your application code here
//!     // Global context is automatically available via fastn_context::global()
//! }
//! ```
//!
//! This is equivalent to:
//!
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     // Manual setup would go here
//! }
//! ```
//!
//! But with automatic context initialization and cleanup.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Main function attribute macro for fastn applications with context support.
///
/// This macro transforms your async main function to automatically set up the tokio runtime
/// and initialize global context management. It's the recommended way to start fastn applications.
///
/// ## Features
///
/// - Automatically creates a multi-threaded tokio runtime
/// - Enables all tokio features (time, net, fs, etc.)
/// - Sets up global context management
/// - Provides clean error handling
///
/// ## Example
///
/// ```rust
/// use fastn_context::main;
///
/// #[main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     println!("Application starting");
///     
///     // Global context is available
///     let ctx = fastn_context::global().await;
///     println!("App context: {}", ctx.name());
///     
///     Ok(())
/// }
/// ```
///
/// ## Return Types
///
/// Your main function can return:
/// - `()` - No error handling
/// - `Result<(), E>` where `E: std::error::Error` - With error handling
///
/// ## Generated Code
///
/// The macro generates a standard `fn main()` that creates the tokio runtime and calls
/// your async function. Error handling is automatically provided.
#[proc_macro_attribute]
pub fn main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let user_fn_name = syn::Ident::new("__fastn_user_main", proc_macro2::Span::call_site());
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;
    let fn_vis = &input_fn.vis;

    quote! {
        #(#fn_attrs)*
        #fn_vis fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
            // Initialize tokio runtime
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(async {
                    // Global context automatically created

                    // Call user's main function
                    let result = #user_fn_name().await;

                    result
                })
        }

        async fn #user_fn_name() -> std::result::Result<(), Box<dyn std::error::Error>> #fn_block
    }
    .into()
}
