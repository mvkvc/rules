# Rust High-Level Coding Guidelines

## Cloning Strategy
- Prefer `.clone()` in non-performance-critical paths when it improves readability
- Use `Arc::clone` liberally for shared ownership when it simplifies code structure
- Consider cloning small, cheap-to-copy types (like primitives) freely
- Document performance-sensitive areas where cloning should be avoided

## Ownership Pragmatism
- Balance ownership strictness with code clarity
- Accept some cloning in business logic if it makes control flow simpler
- Prefer clear, obvious code over clever ownership tricks in most application code
- Use `Rc`/`Arc` when shared ownership better models the domain

## Error Handling
- Use `.unwrap()`/`.expect()` in non-critical paths where errors are truly unrecoverable
- Prefer simple error handling in application code (e.g., anyhow) over complex custom enums
- Document where panics might occur in public APIs

## Performance Tradeoffs
- Optimize only after profiling shows bottlenecks
- Prefer clear code over micro-optimizations in business logic
- Use cloning as a first approach, then optimize if needed
- Document performance-sensitive sections clearly

## Concurrency
- Use `Arc<Mutex>` when shared mutable state is the clearest solution
- Prefer simple parallelism with `rayon` where applicable
- Consider message passing when logic is complex, even if slightly less performant

## Testing
- Use cloning freely in tests to keep them simple
- Prefer clear, readable test cases over perfect ownership
- Document performance characteristics in benchmarks

## Style Guidelines
- Comment where cloning is used for clarity rather than necessity
- Document performance tradeoffs in strategic decisions
- Keep high-level code focused on business logic clarity
- Isolate performance-critical sections clearly
