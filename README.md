# smacro

A collection of convenient Rust macros to reduce boilerplate and improve ergonomics.

## Macros

### `s!`

A versatile macro for creating `String` instances with minimal syntax.

```rust
use smacro::s;

// Empty string
let empty = s!();

// Convert anything with ToString
let hello = s!("Hello, world!");
let number = s!(42);

// Format strings - same as `format!()`
let name = "Alice";
let greeting = s!("Hello, {}!", name);
```

## License

MIT
