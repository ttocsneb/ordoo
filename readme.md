[![docs.rs](https://img.shields.io/docsrs/ordoo)](https://docs.rs/ordoo/latest/ordoo/)

This crate simply adds a macro that makes the syntax simpler to end control
flow.

The macro or_do will allow you to return a function early if a value is
None or Err.

```rs
use ordoo::or_do;

let val: i32 = or_do!(Some(1), return);

let val: i32 = or_do!(Ok::<_, std::io::Error>(1), _ => return);
```

I may add more QoL macros/functions later.
