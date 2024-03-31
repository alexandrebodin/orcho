use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

use log::debug;

use serde::{Deserialize, Serialize};

pub struct Project {
    pub dir: PathBuf,
    pub config: Option<Config>,
    pub package_json: PackageJson,
    pub packages: Option<Vec<Package>>,
    pub is_workspace: bool,
}

pub struct Package {
    pub dir: PathBuf,
    pub package_json: PackageJson,
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub peer_dependencies: Option<HashMap<String, String>>,
    pub optional_dependencies: Option<HashMap<String, String>>,
    pub scripts: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tasks: HashMap<String, Task>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub depends_on: Option<Vec<String>>,
    pub run: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

fn find_project_root_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = env::current_dir().unwrap();
    let mut last_match = None;
    let orcho_files = ["orcho.yaml", "orcho.json"];

    loop {
        for file in orcho_files.iter() {
            let orcho_path = path.join(file);
            if orcho_path.exists() {
                last_match = Some(path.clone());
                break;
            }
        }

        match path.parent() {
            Some(parent_path) => path = parent_path.to_path_buf(),
            None => break,
        }
    }

    return match last_match {
        Some(path) => Ok(path),
        None => Err("No orcho config found".into()),
    };
}

fn load_package_json(root_dir: &Path) -> Result<PackageJson, Box<dyn std::error::Error>> {
    let package_json_path = root_dir.join("package.json");
    if !package_json_path.exists() {
        return Err("No package.json found".into());
    }

    let package_json = std::fs::read_to_string(package_json_path).unwrap();
    let value: PackageJson = serde_json::from_str(&package_json)?;

    debug!("PackageJson: {:?}", package_json);

    Ok(value)
}

fn load_config(root_dir: &Path) -> Result<Option<Config>, Box<dyn std::error::Error>> {
    for file in ["orcho.yaml", "orcho.json"].iter() {
        let orcho_path = root_dir.join(file);
        if orcho_path.exists() {
            let config = std::fs::read_to_string(orcho_path).unwrap();
            let config: Config = serde_yaml::from_str(&config)?;

            debug!("Config: {:?}", config);

            return Ok(Some(config));
        }
    }

    return Ok(None);
}

fn resolve_project(root_dir: &Path) -> Result<Project, Box<dyn std::error::Error>> {
    let pkg = load_package_json(root_dir)?;
    let cfg = load_config(root_dir)?;

    Ok(Project {
        dir: root_dir.to_path_buf(),
        config: cfg,
        package_json: pkg,
        packages: None,
        is_workspace: false,
    })
}

pub fn load() -> Result<Project, Box<dyn std::error::Error>> {
    // find project root dir
    let root_dir = find_project_root_dir()?;

    debug!("Root dir: {:?}", root_dir);

    return resolve_project(&root_dir);
}
