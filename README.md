# fastn-context

Type-safe async context propagation for Rust applications.

## Overview

`fastn-context` provides a clean, efficient way to manage async operation context and cancellation in Rust applications. It's designed for applications that need to track operation status, handle graceful shutdowns, and manage background tasks.

## Features

- **Type-safe context management** with structured data
- **Graceful cancellation** using tokio's CancellationToken
- **Operation tracking** with status and metrics
- **Minimal overhead** - zero-cost when not used
- **Easy integration** with existing async code

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
fastn-context = "0.1"
```

### Basic Usage

```rust
use fastn_context::Context;

#[fastn_context::main]
async fn main() -> eyre::Result<()> {
    // Your async code here
    // Context is automatically available and managed
    Ok(())
}
```

### With Custom Context

```rust
use fastn_context::{Context, ContextStatus};

async fn my_operation(ctx: &Context) -> Result<String, Error> {
    // Check for cancellation
    ctx.cancelled().await;
    
    // Update operation status
    ctx.update_status("Processing data...").await;
    
    // Your business logic
    Ok("result".to_string())
}
```

## Documentation

- [API Documentation](https://docs.rs/fastn-context)
- [Examples](./fastn-context/examples/)

## Use Cases

- **Web servers** - Request context and graceful shutdown
- **Background services** - Task coordination and cancellation  
- **CLI tools** - Operation status and signal handling
- **Distributed systems** - Operation tracking and coordination

## License

Licensed under the Universal Permissive License (UPL-1.0).

## Contributing

Contributions welcome! Please open an issue to discuss major changes before submitting a PR.