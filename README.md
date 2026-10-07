# WordCraft Studio

An open-source, sovereign word processing and technical document editor built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![WordCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode word processor UI with paged canvas, paragraph styling, and contextual action shelf.
- **`crates/engine`**: Lossless DOCX reading and writing, real-time pagination, and multi-language spellcheck integration.

## Legal & Compliance Notice

WordCraft is an independent open-source document editor. It is not affiliated with Microsoft Corporation. Microsoft, Microsoft Word, and Office are trademarks of Microsoft Corporation. All typography controls and document formatting follow open standards (ISO/IEC 29500).

## License

Dual-licensed under MIT OR Apache-2.0.
