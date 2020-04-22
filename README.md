
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

- resources...

    - <https://www.tutorialspoint.com/rust/index.htm>

    - <https://cheats.rs/>

    - [rust-by-example](https://doc.rust-lang.org/stable/rust-by-example/error/result.html)

---


##### things that make rust safer...

- actionable type-annotations; function-parameters require them, and if a function will return a value, it must specify a type-annotation return-type.

- variable mutability must be explicitly set on initialization

- is the integer-overflow wrapping for safety? (Weeks later, don't remember what this refers to!  :/)

- [enums](https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html) -- cool -- I can see how the "invitation" to easily embed different data-types in a fundamental structure could lead to better/safer code.

    - yup, later in this page (`Option` is an enum)... """The Option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing. Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling; this functionality can prevent bugs that are extremely common in other programming languages."""

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

- printing types...

    - hack; where `x` is the variable whose type you want to check:

            let zz: () = x;  // (odd) way of determining a variable type; will throw a "mismatched types" error, and show the expected type of `x` """

    - why this hack? from [this post](https://users.rust-lang.org/t/how-can-i-print-the-type-of-a-variable/4183/6)...

            ...Printing the name of a type at runtime is complicated in Rust, largely
            because types don't really exist at runtime. In a language like Java, objects
            contain class pointers that indicate their type, and you can often reach the
            type's name through them (o.getClass().getName() if I remember my Java
            correctly). Rust doesn't include type information in every value, so that
            approach is right out.

            There are some alternatives, but none is perfect...

- from the struct chapter... _"...Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type."_

    - the import, if I can generalize from this: mentions of "Type" can be thought of (like Structs) as classes (though a Struct doesn't directly contain implementation-methods).

- concurrency

    - <https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html>

    - <https://lib.rs/crates/rayon>

    - <https://blog.yoshuawuyts.com/async-http/>

    - <https://blog.yoshuawuyts.com/streams-concurrency/>

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


At:

- <https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html>

- at beginning.

---


