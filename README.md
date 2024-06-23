# `Ordinal` types and collections

<!-- cargo-rdme start -->

The library provides `Ordinal` trait to map types to `usize` values,
proc-macro to derive `Ordinal` trait for structs and enums,
and `map` and `set` implementations
that use these types as keys efficiently.

## Example

```rust
use ordinal_map::map::OrdinalInitMap;
use ordinal_map::map::OrdinalMap;
#[derive(ordinal_map::Ordinal)]
enum ErrorCategory {
    Network,
    Disk,
    Logic,
}

fn classify_error(error: &str) -> ErrorCategory {
    // ...
}

let mut error_counts: OrdinalInitMap<ErrorCategory, u64> = OrdinalInitMap::default();


for error in &errors {
    let category = classify_error(error);
    error_counts[category] += 1;
}
```

<!-- cargo-rdme end -->
