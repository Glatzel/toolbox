# Envoy

This crate provides ergonomic conversions between Rust and C string types, including utilities for working with C string pointers, lists, and Rust types.

---

## Traits

### `CStrToString`

Converts C string pointers and slices to Rust `String`.

| Input Type      | Null Pointer Handling | Output           |
| --------------- | --------------------- | ---------------- |
| `*const c_char` | Returns `None`        | `Option<String>` |
| `*mut c_char`   | ^                     | ^                |
| `[c_char]`      | ^                     | ^                |

---

### `CStrListToVecString`

Converts a null-terminated list of C string pointers to a `Vec<String>`.

| Input Type             | Null Pointer Handling | Output        |
| ---------------------- | --------------------- | ------------- |
| `*mut *mut c_char`     | Returns empty `Vec`   | `Vec<String>` |
| `*const *const c_char` | ^                     | ^             |

---

### `ToCString`

Converts Rust strings to C-compatible strings.

| Input Type                    | Output               | Notes                   |
| ----------------------------- | -------------------- | ----------------------- |
| `&str` / `String`             | `CString`            | Panics on interior null |
| `Some(&str)` / `Some(String)` | `CString`            | ^                       |
| `None`                        | `CString::default()` | /                       |

### `ToCStrList`

Converts Rust string slices or vectors to lists of C-compatible strings.

| Input Type                                  | Output                            | Notes                    |
| ------------------------------------------- | --------------------------------- | ------------------------ |
| `&[&str]` / `Vec<String>`                   | `Vec<CString>` / `Vec<*const i8>` | Allocates, must be freed |
| `Option<Vec<&str>>` / `Option<Vec<String>>` | Empty if `None` or empty input    | /                        |

---

## Memory Safety

- Any pointer returned by `to_cstr()` must be reclaimed with `CString::from_raw(ptr as *mut i8)` to avoid memory leaks.
- Lists of pointers returned by `to_cstr_list()` must be individually reclaimed.

---

## Example

```rust
use envoy::{ToCStr, CStrToString};

let ptr = "hello".to_cstr();
assert_eq!(ptr.to_string().unwrap(), "hello");
// SAFETY: free the memory after use
unsafe { let _ = std::ffi::CString::from_raw(ptr as *mut i8); }
```
