# Contributing to Rust Port Scanner

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Code of Conduct

- Use this tool only for authorized testing and educational purposes
- Respect intellectual property and privacy
- Report security vulnerabilities responsibly
- Maintain professionalism and courtesy

## How to Contribute

### Reporting Issues

1. Check if the issue already exists
2. Provide a clear description of the problem
3. Include:
   - Your OS and Rust version (`rustc --version`)
   - Steps to reproduce
   - Expected vs. actual behavior
4. Attach relevant logs or error messages

### Submitting Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Test locally: `cargo test && cargo clippy`
5. Commit with clear messages
6. Push to your fork
7. Open a Pull Request with:
   - Clear description of changes
   - Link to related issues
   - Testing notes

### Code Style

- Follow [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html)
- Use `cargo fmt` for formatting
- Run `cargo clippy` to check for common mistakes
- Add comments for complex logic
- Include tests for new functionality

### Building & Testing

```bash
# Navigate to portscan directory
cd portscan

# Build
cargo build --release

# Check code quality
cargo clippy

# Format code
cargo fmt

# Run tests (if any)
cargo test
```

## Feature Ideas

Areas where contributions are welcome:

- [ ] Async/await support for faster scanning
- [ ] Output formats (JSON, CSV, XML)
- [ ] Progress bar during scans
- [ ] Target file batch processing
- [ ] Service name resolution (port → service mapping)
- [ ] Rate limiting to avoid network impacts
- [ ] Connection attempt logging/debugging
- [ ] Cross-platform compatibility improvements
- [ ] Unit and integration tests
- [ ] Performance optimizations
- [ ] Documentation improvements

## Commit Message Guidelines

Write clear, descriptive commit messages:

```
# Good ✓
git commit -m "Add async scanning support with Tokio"
git commit -m "Fix port range validation bug (fixes #42)"
git commit -m "Improve CLI argument parsing and error messages"

# Avoid ✗
git commit -m "Fix stuff"
git commit -m "Update"
```

## Documentation

Help improve documentation by:

- Clarifying unclear sections in README.md
- Adding new examples to EXAMPLES.md
- Improving error messages
- Creating tutorials or guides
- Fixing typos

## Security Considerations

- Never hardcode credentials or sensitive information
- Validate all user inputs thoroughly
- Keep dependencies up to date
- Test with various input edge cases
- Report security issues privately

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

- Open a GitHub discussion
- Review existing issues
- Check the README.md and SETUP.md
- Look at example code in EXAMPLES.md

Thank you for helping make this project better! 🚀
