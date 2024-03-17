use system::system;

#[test]
fn test() {
    let status = system("echo Hello, world!").expect("Failed to run command.");
    assert_eq!(status.code().unwrap(), 0);
}
