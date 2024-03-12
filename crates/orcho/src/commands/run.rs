use std::result::Result;

use petgraph::visit::Dfs;

use crate::{
    dependency_graph,
    exec::exec_task,
    project::{Project, Task},
};

pub fn exec(project: &Project, task_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let graph = dependency_graph(project);

    let node_idx = match graph.nodes.get(&task_name) {
        Some(node_idx) => node_idx,
        None => return Err(format!("Task {} not found", task_name))?,
    };

    let mut ordered_tasks = Vec::<&Task>::new();

    let mut dfs = Dfs::new(&graph.graph, *node_idx);
    while let Some(visited) = dfs.next(&graph.graph) {
        let task = &graph.graph[visited];
        ordered_tasks.insert(0, task);
    }

    for task in ordered_tasks {
        exec_task(task)?;
    }

    Ok(())
}
