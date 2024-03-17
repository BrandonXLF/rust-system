use std::process::Command;

use system::System;

#[test]
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
