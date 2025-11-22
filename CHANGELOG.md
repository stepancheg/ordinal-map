# Unreleased

# 0.1.10 - 2025-11-22

- More efficient `#[derive(Ordinal)]` code generation (faster to compile)

# 0.1.9 - 2025-08-29

- add `AtomicOrdinal`
- `impl Ordinal for Box<T>`
- `impl Ordinal for Ordering`
- `impl Ordinal for char`

# 0.1.8 - 2025-07-19

- `#[allow(clippy::all)]` in generated code
- `is_empty` functions on maps
- `impl Ordinal` for arrays

# 0.1.7 - 2025-06-02

- Specialize `.last()` and `.count()` for `OrdinalValues`
- Add `either` feature

