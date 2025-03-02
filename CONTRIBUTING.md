# Contributing to Neo Contract RS

Thank you for your interest in contributing to Neo Contract RS! We welcome contributions from everyone who is interested in improving the framework.

## How to Contribute

There are many ways to contribute to Neo Contract RS:

- Reporting bugs and issues
- Suggesting new features or enhancements
- Improving documentation
- Writing code and submitting pull requests
- Helping with testing

## Development Workflow

1. **Fork the Repository**
   
   First, fork the [Neo Contract RS repository](https://github.com/R3E-Network/neo-contract-rs) to your GitHub account.

2. **Clone Your Fork**
   
   ```bash
   git clone https://github.com/YOUR-USERNAME/neo-contract-rs.git
   cd neo-contract-rs
   ```

3. **Create a Branch**
   
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Your Changes**
   
   Make your changes to the codebase. Please follow the coding conventions described below.

5. **Test Your Changes**
   
   Make sure to test your changes thoroughly:
   
   ```bash
   cargo test
   ```
   
   For WebAssembly-specific functionality, test with:
   
   ```bash
   cargo build --target wasm32-unknown-unknown
   ```

6. **Commit Your Changes**
   
   ```bash
   git commit -m "Description of your changes"
   ```

7. **Push to Your Fork**
   
   ```bash
   git push origin feature/your-feature-name
   ```

8. **Create a Pull Request**
   
   Go to the [Neo Contract RS repository](https://github.com/R3E-Network/neo-contract-rs) and create a pull request from your fork.

## Coding Conventions

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use meaningful variable and function names
- Write docstrings for public APIs
- Add tests for new functionality
- Keep commits focused and atomic

## Pull Request Process

1. Ensure your code follows the coding conventions
2. Update the documentation if necessary
3. Add tests for new features
4. Ensure all tests pass
5. Update the README.md with details of changes if appropriate
6. The pull request will be merged once it has been reviewed and approved

## Reporting Bugs

When reporting bugs, please include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Rust and toolchain versions
- Any additional context or screenshots

## Community Guidelines

- Be respectful and considerate of others
- Focus on the issue at hand, not the person
- Be open to feedback and be willing to make changes
- Help others when you can

## License

By contributing to Neo Contract RS, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).

## Questions?

If you have any questions or need help, please open an issue or reach out to the maintainers.

Thank you for contributing to Neo Contract RS!
