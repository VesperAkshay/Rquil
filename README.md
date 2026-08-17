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
- **0.6**: (In Progress) Secrets file loading.
