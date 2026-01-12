# micro

[![Rust Tests](https://github.com/JodusMelodus/micro/actions/workflows/tests.yml/badge.svg)](https://github.com/JodusMelodus/micro/actions/workflows/tests.yml)

`micro` is a lightweight Rust crate providing the `FromDTO` procedural macro. It simplifies the conversion between Data Transfer Objects (DTOs) and your internal domain models by automatically generating `From` trait implementations.

## Features

* **Struct Mapping**: Automatically maps named fields from a DTO to your domain struct.
* **Enum Mapping**: Supports Unit, Unnamed (tuple), and Named enum variants.
* **Collection Support**: Specialized handling for `Vec<T>` and `Option<T>`, automatically calling `.into()` on inner items.
* **Generic Support**: Correctly handles generic types (e.g., `List<T>` or `Response<T>`) and generates appropriate trait bounds.
* **Attribute-Driven**: Define multiple source types for a single domain model using the `#[from(...)]` attribute.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
micro = "<version>" # Update <version> to latest
```

## Usage
### Basic Struct Conversion

If field names match, `FromDTO` generates the boilerplate to convert from your DTO to your Domain model, calling `.into()` on every field to handle nested conversions.

```rust
use micro::FromDTO;

// The DTO (External Source)
pub mod external {
    pub struct UserDTO {
        pub id: i64,
        pub username: String,
    }
}

// The Domain Model
#[derive(FromDTO)]
#[from(external::UserDTO)]
pub struct User {
    pub id: i64,
    pub username: String,
}
```

### Generic Enums
The macro handles generic parameters and complex enum structures. It automatically strips generics from paths in match arms to ensure compatibility with stable Rust.

```rust
#[derive(FromDTO)]
#[from(external::ApiResponse<T>)]
pub enum Response<T> {
    Success(T),
    Empty,
    Error(ErrorWrapper),
}
```

### Advanced Collections
`FromDTO` detects `Vec` and `Option` types to ensure that inner types are converted correctly using the `.into_iter().map(...).collect()` pattern.

```rust
#[derive(FromDTO)]
#[from(external::LibraryDTO)]
pub struct Library {
    pub tags: Vec<String>,           // Handled via .collect()
    pub metadata: Option<Metadata>,  // Handled via .map(Into::into)
}
```

### How it Works
The `FromDTO` macro inspects your struct or enum at compile time and generates a `From<Source>` implementation:
1. **Generic Splitting**: Uses `split_for_impl()` to ensure trait bounds like `impl<T> From<Source<T>>` for `Target<T>` are correctly declared.
2. **Path Cleaning**: It strips generic arguments from enum variants specifically within match arms. This avoids the "qualified paths in this context is experimental" error on stable Rust.
3. **Recursive Conversion**: It assumes that inner types also implement From (or are decorated with `FromDTO`) and chains them using `.into()`.
