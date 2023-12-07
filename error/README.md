# ERROR

Errors are handled with panic and Result on Rust.
Panic will make the program abort.
Whereas Result represent an error which can be recovered from.

## PANIC

When panicking the program aborts.
Rust, by default, will print an error message, unwind, clean up the stack, and quit.
This default behavior can the change with:
```Cargo.toml
[profile.release]
panic = 'abort'
```

## RESULT

Result, represented in fig. 1, allows to handle error without quitting the program.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
> fig. 1

### MATCH ON ERROR

One way to handle error is to match on Result.

```rust
let res = ...;

match res {
    Ok(data) => println!("{data}"),
    Err(_) => return,
}
```

### HELPERS

Rust also provides helper on Result.

- unwrap: panic on err
- expect: panic on err, a msg can be added
- unwrap_or: provide a value used on err
- unwrap_or_default: provide a value used on err
- unwrap_or_else: call a closure on err
