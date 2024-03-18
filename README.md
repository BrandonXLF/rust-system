# system

Crate to easily run shell commands across different platforms, similar to the C `system` command.

## Usage

### `system` and `system_output`

For simple use cases where you just need the result of a system command, the `system` and `system_output` functions can be used.

`system` inherits the stdout, stderr, and stdin from the parent process whereas `system_output` captures stdout and stderr and does not inherit an stdin.

An example of using `system`,

```rust
use system::system;

fn main() {
    // Prints "Hello, world!"
    system("echo Hello, world!").expect("Failed to run command.");
}
```

An example of using `system_output`,

```rust
use system::system_output;

fn main() {
    let out = system_output("echo Hello, world!").expect("Failed to run command.");
    let stdout = String::from_utf8_lossy(&out.stdout);

    #[cfg(target_os = "windows")]
    assert_eq!(stdout, "Hello, world!\r\n");

    #[cfg(not(target_os = "windows"))]
    assert_eq!(stdout, "Hello, world!\n");
}
```

### `std::process::Command::system`

For more complex uses cases where the underlying `Command` has to be modified before running the command, the `system::System` trait is implemented for `Command`.

The trait adds the function `Command::system` to create `Command`s that execute shell commands.

For example,

```rust
use std::process::Command;

use system::System;

fn test() {
    let out = Command::system("echo Hello, world!")
        .output()
        .expect("Failed to run command.");
    let stdout = String::from_utf8_lossy(&out.stdout);

    #[cfg(target_os = "windows")]
    assert_eq!(stdout, "Hello, world!\r\n");

    #[cfg(not(target_os = "windows"))]
    assert_eq!(stdout, "Hello, world!\n");
}
```
