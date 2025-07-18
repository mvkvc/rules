# Ruby General Coding Rules (Functional Style)

## Immutability
- Prefer immutable data structures
- Use `freeze` to prevent modifications
- Avoid mutating methods (those ending with `!`)
- Return new objects instead of modifying existing ones

## Pure Functions
- Functions should have no side effects
- Same input always produces same output
- Avoid external state dependencies
- Keep functions small and focused

## Higher-Order Functions
- Prefer `map`, `reduce`, `select` over loops
- Use blocks/procs for behavior parameterization
- Leverage method chaining for transformations
- Consider lazy evaluation with Enumerator::Lazy

## Functional Patterns
- Use recursion where appropriate
- Prefer expressions over statements
- Use method composition
- Consider Value objects for immutable data

## Testing
- Write pure functions for easy testing
- Test input/output relationships
- Avoid testing side effects
- Use dependency injection for impure operations

## Style Guidelines
- Keep methods small and single-purpose
- Document function contracts clearly
- Prefer functional constructs over imperative
- Minimize mutable state
