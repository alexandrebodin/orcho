# Orcho

## Specs

### CLI

```sh
orcho run task [...args]
orcho list
orcho completion
orcho -h
orcho -v
```

### Features

Ecosystems to support:

- node
- rust
- go

- Detect monorepos & allow agnostic config (manual workspace conf)
- Hierarchical configs
- Dependency Graph
- Task Graph
- Concurrent task running
- Standalone task running (exclude task deps or code deps)
- Task declaration in yaml or a custom file format
- Support folder base & code base tasks
- Support background tasks & watchers
- Support global advanced watch mode
- Support dotenv
- Support "diff" to only run tasks base on git history
- Orcho file can only be at the root or in a workspace
- Bind PATH so you can exec localy installed bins (e.g node_modules/.bin)

### Tasks

- env (global + local)
- task
- task vars (global & locale) (vars translate to cmd params)
- cmd & cmd params
- run other tasks
- importing other files
- silent
- loops
- prompting
- aliases
- input & outputs & manual status checks
- dependent tasks
- Conditional runs
- Provide IDE extensions with a json schema to make the orcho.yaml file easy to write

```yml
env:
  NAME: value

tasks:
  taskName:
    dependsOn:
      - "check"
      - "^build"
    # dependencies:
    #   - "package-x"
    env:
      NAME: value
    # array of inputs paths
    inputs:
      - ""
    # array of outputs paths
    outputs:
      - ""
    run:
      - echo $CONFIG
      - 
```
