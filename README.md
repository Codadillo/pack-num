# pack-num
This is a minimal crate to determine the minimum number of bytes required to store an integer.

For example
```rust
assert_eq!(1, pack_num::u32_packed(0));
assert_eq!(1, pack_num::u32_packed(0xff));
assert_eq!(4, pack_num::u64_packed(0x1000000));
```
