# Wrap-rs
_Simple function wrapping in Rust_

Rewrite a function to call an external wrapper before returning:
```rust
fn plus_1(n: usize) -> usize {
    n + 1
}

#[wrap_fn(plus_1 => usize)]
fn f() -> usize {
    4
}

assert_eq!(f(), 5);
```

You can also change the return type:
```rust
fn ok_if_gt_5(n: usize) -> Result<bool, bool> {
    if n > 5 {
        Ok(true)
    } else {
        Err(false)
    }
}

#[wrap_fn(ok_if_gt_5 => Result<bool, bool>)]
fn _4() -> usize {
    4
}

#[wrap_fn(ok_if_gt_5 => Result<bool, bool>)]
fn _6() -> usize {
    6
}

assert_eq!(_4().unwrap_err(), false);
assert_eq!(_6().unwrap(), true);
```

Or use a wrapper with a generic argument:
```rust
fn stringify(t: impl ToString) -> String {
    t.to_string()
}

#[wrap_fn(stringify => String)]
fn num() -> usize {
    42
}

#[wrap_fn(stringify => String)]
fn boolean() -> bool {
    true
}

assert_eq!(num(), "42".to_string());
assert_eq!(boolean(), "true".to_string());
```
