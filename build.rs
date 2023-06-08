fn main() {
    run_yarn_build();
}

fn run_yarn_build() {
    let mut cmd = std::process::Command::new(get_os_process());
    cmd.stdout(std::process::Stdio::piped());

    cmd.current_dir("frontend");
    cmd.args(["-c", "yarn build"]);

    cmd.output().unwrap();
}

fn get_os_process() -> String {
    if cfg!(target_os = "windows") {
        String::from("powershell.exe")
    } else {
        String::from("/bin/bash")
    }
}
