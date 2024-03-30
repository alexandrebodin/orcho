use std::{collections::HashMap, env, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub tasks: HashMap<String, Task>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub depends_on: Option<Vec<String>>,
    pub run: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

fn file_exists(filename: &str) -> bool {
    let path = env::current_dir().unwrap().join(filename);
    Path::new(&path).exists()
}

fn to_abs_path(path: &str) -> String {
    if path.starts_with("/") {
        return path.to_string();
    }

    let p = env::current_dir().unwrap().join(path);
    p.to_str().unwrap().to_string()
}

pub fn load() -> Result<Project, Box<dyn std::error::Error>> {
    // TODO: find up

    for path in vec!["orcho.yaml", "orcho.yml"] {
        let path = to_abs_path(path);

        if file_exists(&path) {
            let config = std::fs::read_to_string(path)?;
            let project: Project = serde_yaml::from_str(&config)?;

            return Ok(project);
        }
    }

    let json_path = to_abs_path("orcho.json");
    if file_exists(&json_path) {
        let config = std::fs::read_to_string(json_path)?;
        let project: Project = serde_json::from_str(&config)?;

        return Ok(project);
    }

    Err("No orcho config found".into())
}
