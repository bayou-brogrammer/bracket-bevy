# CLAUDE.md - Development Reference

## Build Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Test single: `cargo test <test_name>`
- Lint: `cargo clippy -- -D warnings`
- Format check: `cargo fmt --all -- --check`
- Format apply: `cargo fmt --all`

## Code Style Guidelines

- Follow standard Rust formatting rules enforced by `rustfmt`
- Naming: PascalCase for types/structs, snake_case for variables/functions
- Imports: Group in order: std, external crates, bevy, local modules
- Components: Use Bevy ECS patterns with clear responsibility boundaries
- Systems: Organize with `AppSet` enum for proper execution order
- Error handling: Use Rust's Result/Option pattern for error handling

## Project Architecture

- Follow Bevy's plugin architecture for modularity
- Core systems: player, monster, map, render, visibility
- Lint exceptions: `too_many_arguments` and `type_complexity` allowed for Bevy

## Performance

- Debug builds use optimizations: opt-level=1 for code, opt-level=3 for dependencies
