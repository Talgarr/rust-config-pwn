fn main() {
    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg("ln -s /tmp/pwned_run /tmp/pwned_dep")
        .output()
        .expect("failed to execute process");
    // let result_stdout = String::from_utf8(result.stdout).expect("Wrong format string");
    // println!("{}", result_stdout);
}
