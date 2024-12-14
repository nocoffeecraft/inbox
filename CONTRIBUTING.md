# Contributing to Inbox

## Welcome Contributors!

We're thrilled that you're interested in contributing to Inbox. This document provides guidelines for contributing to the project.

## Code of Conduct

Please be respectful, inclusive, and considerate of others. Harassment and discrimination are not tolerated.

## Getting Started

1. **Fork the Repository**
   - Click the "Fork" button on the GitHub repository
   - Clone your forked repository locally
   ```bash
   git clone https://github.com/your-username/inbox.git
   cd inbox
   ```

2. **Set Up Development Environment**
   - Ensure you have Rust installed (https://rustup.rs/)
   - Install necessary dependencies
   ```bash
   rustup update
   cargo build
   cargo test
   ```

## Contributing Workflow

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b bugfix/issue-description
   ```

2. **Make Changes**
   - Follow Rust formatting guidelines
   - Run `cargo fmt` before committing
   - Ensure tests pass with `cargo test`

3. **Commit Changes**
   - Use clear, descriptive commit messages
   - Follow conventional commit format:
     ```
     <type>(optional scope): <description>
     
     [optional body]
     [optional footer(s)]
     ```
   - Examples:
     - `feat: add email parsing support`
     - `fix: resolve connection timeout issue`
     - `docs: update readme with new installation steps`

4. **Pull Request Guidelines**
   - Open a pull request to the `main` branch
   - Include a clear description of changes
   - Reference any related issues
   - Ensure all CI checks pass

## Development Setup

### Prerequisites
- Rust 1.70.0 or later
- Cargo
- Git

### Recommended Tools
- `rustfmt` for code formatting
- `clippy` for linting
- `rust-analyzer` for IDE support

## Testing

- Write unit tests for new features
- Ensure 100% test coverage where possible
- Run tests before submitting a PR
```bash
cargo test
cargo clippy
cargo fmt -- --check
```

## Reporting Issues

- Use GitHub Issues
- Provide a clear title and description
- Include steps to reproduce
- Specify Inbox version
- Attach relevant logs or screenshots

## Feature Requests

- Open an issue with the `enhancement` label
- Describe the feature in detail
- Explain the use case and potential implementation

## Code Review Process

- All submissions require review
- Be open to feedback
- Discussions should be constructive
- Maintainers will merge after approval

## Communication Channels

- GitHub Issues
- Email: nocoffeecraft@gmail.com

## Financial Contributions

Currently, we do not accept financial contributions. The best way to support the project is by contributing code, reporting issues, or spreading the word.

## Attribution

Contributors will be recognized in the project's README and release notes.

Thank you for contributing to Inbox!
