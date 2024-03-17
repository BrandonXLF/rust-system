# system

Crate to easily run system/shell commands across different platforms, similar to the C `system` command.

## Usage

### `system`

For simple use cases where you just need the output of a system command, the `system` can be used.

```rust
use system::system;

fn main() {
    let out = system("echo Hello, world!").expect("Failed to run command.");

    #[cfg(target_os = "windows")]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\r\n");

    #[cfg(not(target_os = "windows"))]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\n");
}
```

### `std::process::Command::system`

For more complex uses cases where the underlying `Command` has to be modified before running the command, the `system::System` trait is provided for `Command`.

The trait adds the function `Command::system` to create `Command`s that execute system/shell commands.

For example,

```rust
    let out = Command::system("echo Hello, world!")
        .output()
        .expect("Failed to run command.");

    #[cfg(target_os = "windows")]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\r\n");

    #[cfg(not(target_os = "windows"))]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\n");
```
