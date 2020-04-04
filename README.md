
---

### Notes from ["The (rust) Book"](https://doc.rust-lang.org/stable/book/title-page.html)

(and other random notes)

---

##### random...

- if i want to have a 'code' and 'binaries' directory in the 'stuff_dir', and I'm in 'code', I can compile with:

        $ rustc --out-dir ../binaries/ ./hello_world.rs

- cargo

    - nice, outputs to a separate directory

    - i like that `cargo check` can check without auto-compiling

    - i like that `cargo run` can compile-_and_-build

    - so cool...

            $ cargo doc --open

- interesting articles

    - [One Program Written in Python, Go, and Rust](https://www.nicolas-hahn.com/python/go/rust/programming/2019/07/01/program-in-python-go-rust/#performance)

---


##### things that make rust safer...

- actionable type-annotations; function-parameters require them, and if a function will return a value, it must specify a type-annotation return-type.

- variable mutability must be explicitly set on initialization

- is the integer-overflow wrapping for safety?

---


##### concepts...

- shadowing vs mut

    - a variable initialized with mut can't change it's type

    - `let var_w_same_name` _does_ let the new var have a different type

- a called function can be before or after the calling function.

- "Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value."

- not rust-specific, but an example of a 'generic' function -- <https://doc.rust-lang.org/book/ch10-02-traits.html#fixing-the-largest-function-with-trait-bounds>. If I'm understanding other reading I've done recently, this is tricky or not possible in Go, where function-parameters have to be explicitly defined.

---

---

currently at:

<https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html>

At beginning.


---


