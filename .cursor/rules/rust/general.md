# Rust General Coding Rules

## Ownership & Borrowing
- Understand and follow Rust's ownership rules strictly
- Prefer borrowing over cloning where possible
- Use references (&) for read-only access
- Use mutable references (&mut) when modification is needed
- Avoid unnecessary .clone() calls
- Do not 

## Error Handling
- Prefer Result over panic for recoverable errors
- Use the ? operator for error propagation
- Implement proper error types using thiserror or anyhow
- Document possible error conditions

## Memory Safety
- Leverage Rust's borrow checker for memory safety
- Use RAII (Resource Acquisition Is Initialization) pattern
- Prefer stack allocation over heap where possible
- Use Box<T> for heap allocation when needed

## Performance
- Prefer zero-cost abstractions
- Use iterators for efficient collection processing
- Consider using &str over String for string slices
- Use #[inline] judiciously for small, hot functions

## Concurrency
- Leverage Rust's thread safety guarantees
- Prefer message passing (channels) over shared state
- Use Arc<Mutex<T>> for shared mutable state
- Consider using crossbeam for advanced concurrency patterns

## Async Programming
- Use Tokio as the default async runtime
- Prefer async/await syntax over manual futures
- Use #[tokio::test] for async tests
- Understand and properly use .await points
- Consider using tokio::spawn for concurrent tasks

## Testing
- Write unit tests in the same file as the code
- Use integration tests for public API testing
- Document test cases clearly
- Use #[test] and #[should_panic] attributes

## Style Guidelines
- Follow Rustfmt formatting
- Use clippy for linting
- Document public APIs with /// comments
- Keep functions small and focused
- Prefer explicit over implicit
