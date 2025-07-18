---
description: Kotlin Web Rules
globs: "**/*.kt"
alwaysApply: false
---

# Kotlin Web Development Rules

## Web Framework Specifics (Ktor)
1. Route definitions should be clear and nested logically
2. Use dependency injection for service classes
3. Prefer suspending functions for IO operations
4. Use structured logging with MDC context
5. Use typed templating (kotlinx.html recommended)
6. Prefer HTMX for client-side interactivity

## Web Framework Specifics (Spring)
1. Keep controllers focused on request/response handling
2. Leverage Spring's dependency injection but keep it simple
3. Keep annotations minimal and meaningful
4. Use Spring Boot starters judiciously
5. Consider Kotlin extensions for Spring where they simplify code

## Error Handling
1. Use sealed classes for domain-specific errors
2. Implement proper HTTP status codes in responses
3. Include meaningful error messages in responses

## Testing
1. Write unit tests for all business logic
2. Use mockk for mocking dependencies
3. Include integration tests for critical API paths
