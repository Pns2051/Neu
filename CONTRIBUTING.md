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
2. **Setup**: Run `cargo build` in the workspace root to ensure your environment is working. (See `README.md` for dependencies like `slint`).
3. **Develop**: Make your changes. Ensure you adhere to the `plugin_sdk` for new sources and the `StreamBuffer` traits for playback enhancements.
4. **Test**: Run `cargo test` and `cargo clippy`. Ensure `cargo fmt` has been applied.
5. **PR**: Open a Pull Request! Detail exactly what your code does and confirm it was reviewed heavily if AI-assisted.

## Architecture Guidelines
When contributing to the core engine:
- Do not bypass the `MusicPlugin` traits.
- For YouTube plugins, ensure the cipher cache logic remains non-blocking.
- UI elements in `app/ui/main.slint` should stick to the MD3 standard design tokens.

We look forward to your contributions!
