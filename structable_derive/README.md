# StructTable derive macro

Most likely you do not want to use this crate directly. It is a helper for the OpenStack

This crate implements derive macros for converting structures
(or structure vectors) as tables (vector of vector of strings -
as rows and columns).

```rust
use structable_derive::StructTable;
#[derive(Serialize, StructTable)]
struct User {
    #[structable(title = "ID")]
    id: u64,
    first_name: &'static str,
    last_name: &'static str,
    #[structable(title = "Long(only in wide mode)", wide)]
    extra: &'static str,
    #[structable(optional, pretty)]
    complex_data: Option<Value>
}
```
