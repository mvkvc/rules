---
description: Rust high-level development
globs: "**/*.rs"
alwaysApply: true
---

- Use `Arc::clone` for shared ownership when it simplifies code structure
- Consider cloning small, cheap-to-copy types (like primitives) freely
- Document performance-sensitive areas where cloning should be avoided
- Use `Arc::clone` liberally for shared ownership when it simplifies code structure
- Prefer `.clone()` in non-performance-critical paths when it improves readability