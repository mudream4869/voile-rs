fn main() {
    println!("cargo:rerun-if-changed=./frontend");
    run_yarn_build();
}

fn run_yarn_build() {
    let mut cmd = std::process::Command::new(get_os_process());
    cmd.stdout(std::process::Stdio::piped());

    cmd.current_dir("frontend");
    cmd.args(["-c", "yarn", "build"]);

    let status = cmd.status().expect("fail execute yarn build frontend");
    if !status.success() {
        panic!("fail to yarn build: {:?}", cmd.output());
    }
}

fn get_os_process() -> String {
    if cfg!(windows) {
        String::from("powershell.exe")
    } else {
        String::from("/bin/bash")
    }
}
