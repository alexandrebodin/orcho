use std::{
    io,
    process::{Child, Command, Stdio},
};

pub fn exec_cmd(cmd: &str) -> io::Result<Child> {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
}
