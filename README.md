# Rust Iterator Invalidation Example

This repository demonstrates a common error in Rust: modifying a vector while iterating over it using an iterator. This can lead to unexpected behavior and panics.

## The Problem

Rust's iterators are tied to the underlying data. If the data is modified while the iterator is active, the iterator can become invalid and accessing it further will result in unpredictable consequences.  This example shows how trying to `push` a new element into the vector after initiating the iterator leads to incorrect or failed access.