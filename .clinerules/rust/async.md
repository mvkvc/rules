---
description: Rust async development
globs: "**/*.rs"
alwaysApply: true
---

- Use Tokio as the default async runtime
- Prefer async/await syntax over manual futures
- Use `#[tokio::test]` for async tests
- Consider using `tokio::spawn` for concurrent tasks
- Understand and properly use `.await` points
