# micro

[![Rust Tests](https://github.com/JodusMelodus/micro/actions/workflows/tests.yml/badge.svg)](https://github.com/JodusMelodus/micro/actions/workflows/tests.yml)

[![GitHub stars](https://img.shields.io/github/stars/JodusMelodus/minmath)](https://github.com/JodusMelodus/micro/stargazers)
[![GitHub license](https://img.shields.io/github/license/JodusMelodus/micro)](https://github.com/JodusMelodus/micro/LICENSE.md)

[![Crates.io](https://img.shields.io/crates/v/micro.svg)](https://crates.io/crates/micro)
[![Crates.io downloads](https://img.shields.io/crates/d/micro.svg)](https://crates.io/crates/micro)
[![Docs.rs](https://docs.rs/minmath/badge.svg)](https://docs.rs/micro)

[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org/)

`micro` is a lightweight Rust crate providing the `FromDto` procedural macro. It simplifies the conversion between Data Transfer Objects (DTOs) and your internal domain models by automatically generating `From` trait implementations.

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

If field names match, `FromDto` generates the boilerplate to convert from your DTO to your Domain model, calling `.into()` on every field to handle nested conversions.

```rust
use micro::FromDto;

// The DTO (External Source)
pub mod external {
    pub struct UserDTO {
        pub id: i64,
        pub username: String,
    }
}

// The Domain Model
#[derive(FromDto)]
#[from(external::UserDTO)]
pub struct User {
    pub id: i64,
    pub username: String,
}
```

### Generic Enums
The macro handles generic parameters and complex enum structures. It automatically strips generics from paths in match arms to ensure compatibility with stable Rust.

```rust
#[derive(FromDto)]
#[from(external::ApiResponse<T>)]
pub enum Response<T> {
    Success(T),
    Empty,
    Error(ErrorWrapper),
}
```

### Advanced Collections
`FromDto` detects `Vec` and `Option` types to ensure that inner types are converted correctly using the `.into_iter().map(...).collect()` pattern.

```rust
#[derive(FromDto)]
#[from(external::LibraryDTO)]
pub struct Library {
    pub tags: Vec<String>,           // Handled via .collect()
    pub metadata: Option<Metadata>,  // Handled via .map(Into::into)
}
```
### Nested Collections (Option + Vec)
Converting APIs often involves deeply nested optional collections, such as `Option<Vec<T>>`. Since Rust's `Into` trait does not automatically reach through multiple layers of containers, `FromDto` detects these patterns and generates the necessary mapping code automatically.

```rust
#[derive(FromDto)]
#[from(external::TrackDTO)]
pub struct Track {
    // Generates: value.contributors.map(|v| v.into_iter().map(Into::into).collect())
    pub contributors: Option<Vec<Artist>>, 
}
```

### How it Works
The `FromDto` macro inspects your struct or enum at compile time and generates a `From<Source>` implementation:
1. **Generic Splitting**: Uses `split_for_impl()` to ensure trait bounds like `impl<T> From<Source<T>>` for `Target<T>` are correctly declared.
2. **Path Cleaning**: It strips generic arguments from enum variants specifically within match arms. This avoids the "qualified paths in this context is experimental" error on stable Rust.
3. **Recursive Conversion**: It assumes that inner types also implement From (or are decorated with `FromDto`) and chains them using `.into()`.
