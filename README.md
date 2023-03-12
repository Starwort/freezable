# `freezable`

`freezable` is a simple library for freezing data in Rust.

Objects which can be frozen implement the `Freezable` trait, which provides a `freeze` method. This returns a `Frozen<T>`, which wraps the frozen type. The `Frozen<T>` type implements all the comparison traits that `T` implements, as well as `Index`, `Clone`, and `Debug`, if the inner type implements them.

## Why use this library?

The primary motivation for this library was frozen versions of `HashMap` and `HashSet` which support `Hash`. This allows them to be used as keys in other `HashMap`s and `HashSet`s.
