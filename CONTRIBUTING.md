# Contributing to Music Client

First off, thank you for considering contributing to the Music Client! It's people like you that make open source such a great community to learn, inspire, and create.

## 🤖 AI Assisted Development Policy

We embrace modern tooling! **AI tools (like Copilot, ChatGPT, Claude) are permitted** for development in this project. 

However, we have a strict quality policy:
- **Human Supervision Required**: All AI-authored code *must* be heavily reviewed, understood, and audited by an experienced programmer before submission.
- **Accountability**: You are completely responsible for the code you submit. "The AI wrote it" is not an acceptable excuse for bugs, memory leaks, or architectural violations.
- **Licensing Compliance**: Ensure that any AI output does not infringe on existing copyrighted code.

## How Can I Contribute?

### Reporting Bugs
This section guides you through submitting a bug report.
- Check if the bug has already been reported in the Issues tab.
- Describe the bug clearly with reproduction steps.
- Include OS, Rust version, and App version.

### Suggesting Enhancements
- Check if the enhancement has already been requested.
- Provide a clear motivation and describe the intended behavior.

### Your First Code Contribution
1. **Fork & Branch**: Fork the repo and create a new branch (`git checkout -b feature/your-feature-name`).
2. **Setup**: 
    - For Rust: Run `cargo build` in the workspace root.
    - For C++: Ensure Qt6 (Core, Gui, Qml, Quick, Network, Multimedia, Sql, NetworkAuth) is installed. Run `cmake -B build && cmake --build build`.
3. **Develop**: 
    - **Extensions**: If you are adding a new music source, follow the [PLUGINS.md](PLUGINS.md) guide to create a C++ extension.
    - **Core**: Ensure you adhere to the `MusicPlugin` interface for new sources.
4. **Test**: 
    - Rust: Run `cargo test` and `cargo clippy`.
    - C++: Ensure the build completes without errors.
5. **PR**: Open a Pull Request! Detail exactly what your code does.

## Architecture Guidelines
When contributing to the core engine:
- Do not bypass the `MusicPlugin` traits.
- For YouTube plugins, ensure the cipher cache logic remains non-blocking.
- UI elements in `app/ui/main.slint` should stick to the MD3 standard design tokens.

We look forward to your contributions!
