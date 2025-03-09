# Contributing to Neo Compiler

Thank you for your interest in contributing to the neo-compiler project! This document provides guidelines and instructions for contributing to this repository.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [How to Contribute](#how-to-contribute)
4. [Development Workflow](#development-workflow)
5. [Pull Request Process](#pull-request-process)
6. [Coding Standards](#coding-standards)
7. [Testing Guidelines](#testing-guidelines)
8. [Documentation Guidelines](#documentation-guidelines)
9. [Community](#community)

## Code of Conduct

We expect all contributors to adhere to the following principles:

- Be respectful and inclusive
- Be collaborative and constructive
- Focus on what is best for the community and the project
- Gracefully accept constructive criticism

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo
- Git

### Setting Up the Development Environment

1. **Fork the Repository**
   
   Start by forking the repository on GitHub:
   https://github.com/neo-project/neo-contract-rs

2. **Clone Your Fork**
   
   ```bash
   git clone https://github.com/your-username/neo-contract-rs.git
   cd neo-contract-rs
   ```

3. **Add the Upstream Remote**
   
   ```bash
   git remote add upstream https://github.com/neo-project/neo-contract-rs.git
   ```

4. **Install Development Dependencies**
   
   ```bash
   cargo build
   ```

## How to Contribute

There are many ways to contribute to the project:

1. **Reporting Bugs**
   
   Open an issue in the GitHub repository. Please include:
   - A clear, descriptive title
   - A detailed description of the issue
   - Steps to reproduce the behavior
   - Expected behavior
   - Any relevant logs or error messages

2. **Suggesting Enhancements**
   
   Open an issue with your enhancement suggestion. Please include:
   - A clear, descriptive title
   - A detailed description of the suggested enhancement
   - The rationale for implementing this enhancement
   - Possible implementation approach, if known

3. **Code Contributions**
   
   See the [Development Workflow](#development-workflow) section below.

4. **Documentation Improvements**
   
   Documentation is just as important as code. If you find areas where the documentation could be improved, please submit a PR.

5. **Answering Questions**
   
   Help answer questions from other users in issues or discussions.

## Development Workflow

1. **Choose an Issue**
   
   Start by looking at open issues in the GitHub repository. Issues labeled "good first issue" are a good starting point for new contributors.

2. **Create a Branch**
   
   ```bash
   git checkout -b feature/your-feature-name
   ```
   
   Use a descriptive branch name that reflects the changes you're making.

3. **Make Your Changes**
   
   Make your changes to the codebase. Be sure to follow the [Coding Standards](#coding-standards).

4. **Write Tests**
   
   Add tests for your changes. See [Testing Guidelines](#testing-guidelines) for more details.

5. **Run Tests Locally**
   
   ```bash
   cargo test
   ```

6. **Update Documentation**
   
   If your changes affect the public API or user-facing functionality, update the relevant documentation.

7. **Commit Your Changes**
   
   ```bash
   git add .
   git commit -m "Clear description of your changes"
   ```
   
   Write clear, concise commit messages that explain the reasons for your changes.

8. **Keep Your Branch Updated**
   
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

9. **Push Your Changes**
   
   ```bash
   git push origin feature/your-feature-name
   ```

10. **Submit a Pull Request**
    
    See the [Pull Request Process](#pull-request-process) section.

## Pull Request Process

1. **Create a Pull Request**
   
   Go to the GitHub repository and create a new pull request from your feature branch to the main branch of the original repository.

2. **Describe Your Changes**
   
   In the pull request description, include:
   - A reference to the issue(s) your PR addresses
   - A summary of the changes you've made
   - Any notes on implementation details that reviewers should be aware of
   - Screenshots or examples if applicable

3. **Wait for Review**
   
   Project maintainers will review your PR. Be responsive to feedback and make changes as requested.

4. **Address Review Comments**
   
   If changes are requested during the review process, make the necessary updates and push them to your branch.

5. **Merge**
   
   Once your PR is approved, a project maintainer will merge it into the main codebase.

## Coding Standards

We follow Rust's standard coding practices with some project-specific guidelines:

1. **Code Style**
   
   - Follow [Rust's naming conventions](https://rust-lang.github.io/api-guidelines/naming.html)
   - Use meaningful, descriptive names for variables, functions, and types
   - Keep functions focused on a single responsibility
   - Format your code with `rustfmt`

2. **Documentation**
   
   - All public APIs should have documentation comments
   - Use examples in doc comments where appropriate
   - Keep documentation up-to-date with code changes

3. **Error Handling**
   
   - Use the project's `Error` type defined in `error.rs` for error handling
   - Avoid unwrap() and expect() in production code
   - Use meaningful error messages

4. **Performance**
   
   - Be mindful of memory usage and allocation
   - Consider optimization when dealing with potentially large data structures
   - Use benchmarks to verify performance improvements

## Testing Guidelines

1. **Unit Tests**
   
   - Write unit tests for all new functionality
   - Tests should be independent and focused on a single aspect
   - Use meaningful assertions that verify the intended behavior

2. **Integration Tests**
   
   - Write integration tests that verify components work together correctly
   - Include tests for edge cases and error conditions

3. **Running Tests**
   
   ```bash
   # Run all tests
   cargo test
   
   # Run specific tests
   cargo test test_name
   
   # Run tests with debug output
   cargo test -- --nocapture
   ```

## Documentation Guidelines

1. **Code Documentation**
   
   - Use `///` for doc comments on items
   - Document all public items (functions, types, modules)
   - Include examples for complex functionality

2. **User Documentation**
   
   - Update user guides when adding new features
   - Keep examples up-to-date with the latest API
   - Use clear, concise language

3. **README and Main Documentation**
   
   - Keep the project README and main documentation up-to-date
   - Ensure installation instructions work
   - Update version information as needed

## Community

### Getting Help

If you need help with contributing, you can:

1. Open an issue with your question
2. Reach out on the Neo Discord server
3. Ask in the GitHub Discussions section

### Recognition

All contributors will be recognized in the project. We value and appreciate all contributions, whether they're code, documentation, issues, or community support.

---

Thank you for contributing to the neo-compiler project! Your efforts help improve the Neo blockchain ecosystem for everyone. 