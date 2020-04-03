
---

### Notes from ["The (rust) Book"](https://doc.rust-lang.org/stable/book/title-page.html)

---

- if i want to have a 'code' and 'binaries' directory in the 'stuff_dir', and I'm in 'code', I can compile with:

        $ rustc --out-dir ../binaries/ ./hello_world.rs

- cargo

    - nice, outputs to a separate directory

    - i like that `cargo check` can check without auto-compiling

    - i like that `cargo run` can compile-_and_-build

    - so cool...

            $ cargo doc --open

---


- things that make rust safer...

    - actionable type-annotations

    - variable mutability must be explicitly set on initialization

---


concepts...

- shadowing vs mut

    - a variable initialized with mut can't change it's type

    - `let var_w_same_name` _does_ let the new var have a different type

---

currently at:

<https://doc.rust-lang.org/stable/book/ch03-02-data-types.html>

beginning.


---
