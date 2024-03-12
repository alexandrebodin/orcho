use std::collections::HashMap;

use clap::{Parser, Subcommand};
use petgraph::Graph;
use project::Task;

mod commands;
mod exec;
mod project;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run { name: String },
}

pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let project = project::load()?;

    match cli.command {
        Some(Commands::Run { name }) => {
            commands::run::exec(&project, name)
            // read the config file
        }
        None => {
            if let Some(name) = cli.name {
                return commands::run::exec(&project, name);
            }

            Err("No command specified")?
        }
    }
}

#[derive(Debug)]
pub struct TaskGraph {
    pub graph: petgraph::Graph<Task, ()>,
    pub nodes: HashMap<String, petgraph::graph::NodeIndex>,
}

pub fn dependency_graph(project: &project::Project) -> TaskGraph {
    let mut graph = Graph::<Task, ()>::new();
    let mut nodes: HashMap<String, _> = HashMap::new();

    for (name, task) in &project.tasks {
        let node = graph.add_node(task.clone());
        nodes.insert(name.to_string(), node);
    }

    for (name, task) in &project.tasks {
        let from = nodes.get(name).unwrap();

        if let Some(depends_on) = &task.depends_on {
            for dep in depends_on {
                let to = nodes.get(&dep.to_string()).unwrap();

                graph.add_edge(*from, *to, ());
            }
        }
    }

    TaskGraph { graph, nodes }
}
