in
```rust
#[declare::type_union]
mod abc {
    struct A {
        x: u32,
        y: u32,
        z: u32
    }

    struct B {
        x: u32,
        y: u32,
    }

    struct C {
        x: String,
        y: u32,
        z: u32
    }
}

```
out
```rust
enum ABC {
  A(A),
  B(B),
  C(C),
}

enum XField {
    u32(u32),
    String(String),
}

impl ABC {
    fn x() -> XField {
        match self {
            ABC::A(a) => XField::u32(a.x),
            ABC::B(b) => XField::u32(b.x),
            ABC::C(c) => XField::String(c.x),
        }
    }

    fn y() -> u32 {
        match self {
            ABC::A(a) => a.y,
            ABC::B(b) => b.y,
            ABC::C(c) => c.y,
        }
    }

    fn z() -> Option<u32> {
        match self {
            ABC::A(a) => Some(a.z),
            ABC::B(b) => None,
            ABC::C(c) => Some(c.z),
        }
    }
}
```

```rust
fn main() {
  let abc = Abc::A(B{
    x: 1
  });
  let y = abc.y(); // is i32 since all have y, otherwise Option<i32> they did not
  let y_common = abc.y_common(); // enum YCommon { A(A) }
  let y = abc.y().value(); // is i32 since all have y, otherwise Option<i32>
}
```