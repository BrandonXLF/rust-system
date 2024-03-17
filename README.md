# System

Trait for the `Command` struct to conveniently run system commands, like the C system function.

## Example

```rust
use std::process::Command;

// Import System trait to use Command::system
use system::System;

fn main() {
    let _ = Command::system("echo Hello, world! && echo 123").status();
}
```

Prints `Hello, world!` and `123`.
