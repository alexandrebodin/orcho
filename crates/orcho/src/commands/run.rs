use std::result::Result;

use crate::{exec::exec_task, project::Project};

pub fn exec(project: &Project, task_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let task = match project.tasks.get(&task_name) {
        Some(task) => task,
        None => return Err(format!("Task {} not found", task_name))?,
    };

    exec_task(task)
}
