# Envoy

## CStrToString

| `*const i8 / *mut i8 / [i8]` | `null` |
| ---------------------------- | ------ |
| `String`                     | `None` |

## CStrListToVecString

| `*mut *mut i8 / *const *const i8` | `null` |
| -------------- | ------ |
| `Vec<String>`  | `None` |

## ToCStr

### to_cstring()

| `&str / String` | `Some(&str) / Some(String)` | `None`               |
| --------------- | --------------------------- | -------------------- |
| `CString`       | `CString`                   | `CString::default()` |

### to_cstr()

| `&str / String` | `Some(&str) / Some(String)` | `None` |
| --------------- | --------------------------- | ------ |
| `*const i8`     | `*const i8`                 | `null` |
