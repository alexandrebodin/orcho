use std::{collections::BTreeMap, env};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub tasks: BTreeMap<String, Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub run: Vec<String>,
}

pub fn load() -> Result<Project, Box<dyn std::error::Error>> {
    // TODO: support diff extensions
    // TODO: find up
    let path = std::path::Path::new(env::current_dir()?.as_path()).join("orcho.yml");
    let config = std::fs::read_to_string(path)?;
    let project: Project = serde_yaml::from_str(&config)?;

    Ok(project)
}
