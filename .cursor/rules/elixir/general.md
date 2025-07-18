---
description: Elixir General Rules
globs: "**/*.{ex,exs,heex}"
alwaysApply: true
---

# Elixir General Rules

## Naming Conventions
- Module names should be PascalCase (e.g., `UserManager`)
- Function names should be snake_case (e.g., `process_data`)
- Predicate functions should end with `?` (e.g., `valid?`)
- Add `_` prefix to unused variables (e.g., `_unused_var`)

## Pattern Matching
- Prefer pattern matching in function heads over conditional logic
- Use `_` for ignored variables
- Avoid overusing pattern matching in complex nested structures
- Use guards (`when`) to add constraints to pattern matches

## Pipe Operator (`|>`)
- Use pipes to chain transformations clearly
- Avoid mixing pipes with other control flow structures

## Documentation
- Use `@moduledoc` for module-level documentation
- Use `@doc` for public functions
- Include typespecs (`@spec`) for all public functions

## Testing
- Use `ExUnit` assertions (`assert`, `refute`, `assert_raise`, etc.)
- Group related tests with `describe` blocks
- Use `setup` and `setup_all` for test fixtures
- Prefer property-based testing (`StreamData`) where appropriate

## Error Handling
- Use `{:ok, result}` and `{:error, reason}` tuples for expected errors
- Reserve exceptions (`raise`) for truly exceptional cases
- Implement the `Exception` protocol for custom errors
- Use `try/rescue` sparingly - prefer pattern matching
