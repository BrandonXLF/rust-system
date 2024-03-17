use std::process::Command;

use system::System;

#[test]
fn test() {
    let out = Command::system("echo Hello, world!")
        .output()
        .expect("Failed to run command.");

    #[cfg(target_os = "windows")]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\r\n");

    #[cfg(not(target_os = "windows"))]
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Hello, world!\n");
}
