# Contributing to spatial-narrative

Thank you for your interest in contributing to spatial-narrative! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Style Guide](#style-guide)

## Code of Conduct

This project adheres to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A GitHub account

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/spatial-narrative.git
   cd spatial-narrative
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/spatial-narrative.git
   ```

## Development Setup

### Build the Project

```bash
cargo build
```

### Run Tests

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test core
cargo test index
cargo test graph

# Run with verbose output
cargo test -- --nocapture
```

### Generate Documentation

```bash
cargo doc --open
```

### Run Examples

```bash
cargo run --example basic_usage
cargo run --example io_formats
cargo run --example indexing
```

## Making Changes

### Branch Naming

Create a descriptive branch for your changes:

```bash
git checkout -b feature/add-clustering-algorithm
git checkout -b fix/timestamp-parsing-edge-case
git checkout -b docs/improve-api-examples
```

### Commit Messages

Follow conventional commit format:

```
type(scope): short description

Longer description if needed.

Fixes #123
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `refactor`: Code refactoring
- `test`: Adding tests
- `perf`: Performance improvement
- `chore`: Maintenance tasks

Examples:
```
feat(index): add sliding window iterator for temporal queries
fix(io): handle missing elevation field in GeoJSON import
docs(graph): add examples for subgraph extraction
```

## Testing

### Writing Tests

All new features must include tests. Place tests in the same file using a `tests` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_basic() {
        // Test implementation
    }

    #[test]
    fn test_feature_edge_cases() {
        // Edge case testing
    }
}
```

### Test Categories

1. **Unit Tests**: Test individual functions/methods
2. **Integration Tests**: Test module interactions (in `tests/` directory)
3. **Doc Tests**: Examples in documentation that are tested
4. **Property Tests**: Use `proptest` for property-based testing

### Running Specific Tests

```bash
# Run tests matching a pattern
cargo test timestamp

# Run tests in a specific module
cargo test core::timestamp

# Run ignored tests
cargo test -- --ignored

# Run benchmarks
cargo bench
```

## Documentation

### Rustdoc Guidelines

All public items must have documentation:

```rust
/// Brief one-line description.
///
/// Longer description with details about behavior,
/// edge cases, and usage patterns.
///
/// # Arguments
///
/// * `param` - Description of the parameter
///
/// # Returns
///
/// Description of return value.
///
/// # Errors
///
/// Describe error conditions if the function returns Result.
///
/// # Examples
///
/// ```rust
/// use spatial_narrative::core::Location;
///
/// let loc = Location::new(40.7128, -74.0060);
/// assert!(loc.lat > 40.0);
/// ```
///
/// # Panics
///
/// Describe panic conditions if any.
pub fn function_name(param: Type) -> ReturnType {
    // implementation
}
```

### Module Documentation

Each module should have a doc comment at the top of `mod.rs`:

```rust
//! Module-level documentation.
//!
//! # Overview
//!
//! Describe what this module provides.
//!
//! # Example
//!
//! ```rust
//! // Usage example
//! ```
```

## Pull Request Process

### Before Submitting

1. **Update your branch** with the latest upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run the full test suite**:
   ```bash
   cargo test
   cargo test --doc
   ```

3. **Check formatting**:
   ```bash
   cargo fmt --check
   ```

4. **Run clippy**:
   ```bash
   cargo clippy -- -D warnings
   ```

5. **Update documentation** if needed

### PR Description Template

```markdown
## Description

Brief description of changes.

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

Describe testing performed.

## Checklist

- [ ] Tests pass locally
- [ ] Documentation updated
- [ ] Formatting checked (`cargo fmt`)
- [ ] Linting passed (`cargo clippy`)
```

### Review Process

1. A maintainer will review your PR
2. Address any requested changes
3. Once approved, your PR will be merged

## Style Guide

### Rust Style

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

- Use `rustfmt` for formatting
- Follow standard naming conventions:
  - `snake_case` for functions, methods, variables
  - `CamelCase` for types, traits
  - `SCREAMING_SNAKE_CASE` for constants
- Prefer `impl Trait` for return types when appropriate
- Use `Self` in impl blocks

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use the `thiserror` crate for custom errors
- Provide context in error messages
- Avoid `unwrap()` in library code (use `expect()` with message if unavoidable)

### Performance Considerations

- Avoid unnecessary allocations
- Use iterators instead of collecting when possible
- Consider `Cow<str>` for string parameters
- Profile before optimizing

## Getting Help

- Open an issue for bugs or feature requests
- Use discussions for questions
- Tag maintainers for urgent issues

## Recognition

Contributors are recognized in:
- The CHANGELOG for each release
- The GitHub contributors page
- Project documentation

Thank you for contributing!
