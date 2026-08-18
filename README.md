# Rquil (Working Name)

A local-first, Git-native API client written in Rust.

## Architecture

- `relay-core`: The foundational Rust library responsible for parsing `.rl` files, resolving variables, and executing requests. (Currently in development).

## Project Progress

We are building this project incrementally.

### Phase 0: Core engine
- **0.1**: Scaffolded the `relay-core` library crate.
- **0.2**: Defined the Rust data models (`serde`) for the `.rl` TOML file format.
- **0.3**: Implemented the `.rl` file parser with strong error handling.
- **0.4**: Implemented variable interpolation for `{{var}}` syntax in strings.
- **0.5**: Implemented the variable scope resolution engine.
- **0.6**: Implemented secrets file loading and integration.
- **0.7**: Implemented HTTP executor with `reqwest`.
- **0.8**: Implemented unit tests for parsing, resolution, and mock HTTP.
- **0.9**: Implemented manual test against live API, finishing Phase 0.

### Phase 1: CLI (Completed)
- **1.1**: Scaffolded the `relay-cli` crate and configured `clap`.
- **1.2**: Implemented folder walking for `.rl` files.
- **1.3**: Wired `relay run` to `relay-core` for live execution.
- **1.4**: Implemented JUnit XML report generation (`--junit`).
- **1.5**: Implemented JSON output (`--json`).
- **1.6**: Process exit codes based on test success/failure.
- **1.7**: Final manual verification of all output formats and failure conditions.

### Phase 2: Minimal GUI (Tauri)
- **2.1**: Scaffolded the Tauri app (`relay-gui`) with React + Vite + TS, linked `relay-core`.
- **2.2**: Built Tauri command to list a collection's requests.
- **2.3**: Built the frontend collection tree view (Sidebar).
- **2.4**: (In Progress) Frontend: request editor form.
