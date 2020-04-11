
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

- next... <https://stevedonovan.github.io/rust-gentle-intro/>

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

- to read, re error-handling... <https://brson.github.io/2016/11/30/starting-with-error-chain>

- borrowing -- re-read this in 2 days, then 4 days, then a week apart to burn it in: <https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html>

- concurrency

    - <https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html>

    - <https://lib.rs/crates/rayon>

    - <https://blog.yoshuawuyts.com/async-http/>

    - <https://blog.yoshuawuyts.com/streams-concurrency/>

- resources...

    - <https://www.tutorialspoint.com/rust/index.htm>

---

#### logging...

- resources...

    - <https://github.com/rust-lang/log>
        - <https://docs.rs/log/0.4.10/log/>  (docds.rs is the crate-documentation site)

- trying the simple logging example at <https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-an-error-message-to-the-console>

    - put this at the top:

            #[macro_use]
            extern crate log;
            extern crate env_logger;

    - then, near the top of `fn main()` put:

            env_logger::init();
            error!("logger test");

    - output...

            error[E0463]: can't find crate for `log`
             --> src/main.rs:2:1
              |
            2 | extern crate log;
              | ^^^^^^^^^^^^^^^^^ can't find crate

            error: aborting due to previous error

    - solution:

        - go to <https://crates.io>, search on package, and add them to the `Cargo.toml` file under `[dependencies]`.

        - then a `cargo check` or run will auto-import them.


---

---


At... see ownership -> main.rs

---


