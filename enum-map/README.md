A library providing enum map providing type safe enum array. It is
implemented using regular Rust arrays, so using them is as fast (and
in fact faster due to no bounds checking) as using regular Rust
arrays.

# Examples

```rust
#[macro_use]
extern crate enum_map;

use enum_map::EnumMap;

#[derive(Debug, EnumMap)]
enum Example {
    A,
    B,
    C,
}

fn main() {
    let mut map = enum_map! {
        Example::A => 1,
        Example::B => 2,
        Example::C => 3,
    };
    map[Example::C] = 4;

    assert_eq!(map[Example::A], 1);

    for (key, &value) in &map {
        println!("{:?} has {} as value.", key, value);
    }
}
```