# Parser stability improvements (v2)

## Summary

Replaces panicking `expect_expression()` with `try_into_expression()` and improves parser error messages from vague "unexpected token" to `Error::unexpected` with specific context.

## Changes

### 1. Replace `expect_expression` with `try_into_expression`

- **fpl_or_exp.rs**: Removed panicking `expect_expression()`; callers now use `try_into_expression()` which returns `Err` for invalid arrow-function arguments instead of panicking.
- **conditional.rs**: Uses `try_into_expression()?` for conditional expression LHS.
- **left_hand_side/mod.rs**: Uses `try_into_expression()?` for call expression and optional expression.

### 2. Improve error messages

- **class_decl/mod.rs**: `Error::general("unexpected token")` → `Error::unexpected(token, span, "expected class element (method, field, or accessor)")`.
- **for_statement.rs**: For-of loop with `let` as loop variable: `Error::general("unexpected token")` → `Error::unexpected("let", span, "for-of loop cannot have 'let' as the loop variable")`.

## Rationale

- Fewer panics improve stability and debuggability.
- More specific errors improve the developer experience when writing invalid syntax.
