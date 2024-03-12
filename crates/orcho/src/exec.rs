use std::process::{Command, Stdio};

use crate::project::Task;

pub fn exec_task(task: &Task) -> Result<(), Box<dyn std::error::Error>> {
    for cmd_str in &task.run {
        let mut cmd = Command::new("sh");

        cmd.arg("-c").arg(cmd_str);

        if let Some(envs) = &task.env {
            cmd.envs(envs);
        }

        cmd.stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;
    }

    Ok(())
}
