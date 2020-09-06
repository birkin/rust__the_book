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

    - <https://blog.usejournal.com/6-useful-rust-macros-that-you-might-not-have-seen-before-59d1386f7bc5>

    - <https://www.snoyman.com/series/rust-crash-course>

    - <https://github.com/psibi/rust-book-summary>

- printing while running tests...

        cargo test -- --nocapture

- re the iterator filter example in <https://doc.rust-lang.org/stable/book/ch13-02-iterators.html#using-closures-that-capture-their-environment>...

    - why the `fn main() {}` at the bottom? The test runs fine without it.

- `$ cargo metadata | python3 -m json.tool` -- cool!

- `$ cargo doc --open`

---


##### things that make rust safer...

- actionable type-annotations; function-parameters require them, and if a function will return a value, it must specify a type-annotation return-type.

- variable mutability must be explicitly set on initialization

- is the integer-overflow wrapping for safety? (Weeks later, don't remember what this refers to!  :/)

- [enums](https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html) -- cool -- I can see how the "invitation" to easily embed different data-types in a fundamental structure could lead to better/safer code.

    - yup, later in this page (`Option` is an enum)... """The Option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing. Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling; this functionality can prevent bugs that are extremely common in other programming languages."""

- part practice and part language: [using types for validation](https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation)...

    - blurb: """However, having lots of error checks in all of your functions would be verbose and annoying. Fortunately, you can use Rust’s type system (and thus the type checking the compiler does) to do many of the checks for you. If your function has a particular type as a parameter, you can proceed with your code’s logic knowing that the compiler has already ensured you have a valid value. For example, if you have a type rather than an Option, your program expects to have something rather than nothing. Your code then doesn’t have to handle two cases for the Some and None variants: it will only have one case for definitely having a value. Code trying to pass nothing to your function won’t even compile, so your function doesn’t have to check for that case at runtime. Another example is using an unsigned integer type such as u32, which ensures the parameter is never negative."""

- cargo new name-of-project --lib _starts_ with a dummy test.

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

            ...Printing the name of a type at runtime is complicated in Rust, largely because types don't really exist at runtime. In a language like Java, objects contain class pointers that indicate their type, and you can often reach the type's name through them (o.getClass().getName() if I remember my Java correctly). Rust doesn't include type information in every value, so that
            approach is right out.

            There are some alternatives, but none is perfect...

- from the struct chapter... _"...Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type."_

    - the import, if I can generalize from this: mentions of "Type" can be thought of (like Structs) as classes (though a Struct doesn't directly contain implementation-methods).

- concurrency

    - <https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html>

    - <https://lib.rs/crates/rayon>

    - <https://blog.yoshuawuyts.com/async-http/>

    - <https://blog.yoshuawuyts.com/streams-concurrency/>

- struct fields are private by default; enum variants are public by default.

- [good config info](https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html?highlight=constructor#the-trade-offs-of-using-clone)

- surprising... a non-mutable vector can be created, and the next line can attempt to access an out-of-bounds value, and the code will compile (and then fail at runtime). Why is that permitted? It seems that given all the safety features, getting a value from the index could yield a Result or Option.

- wow; the String info on [Internal Representation](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#internal-representation) and on [Bytes and Scalar Values and Grapheme Clusters](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my) is fascinating.

    - One question, though... the docs state that the Hindi word, when stored as "Unicode scalar values", would have 6 `char` values, 2 of which are diacritics. I assume that for the characters than can be encoded either as a single unicode element, or as two unicode elements -- that rust has a way of specifying that. (I think that's 'normalized' or 'denormalized'). After a lookup, yes -- [there are functions](https://doc.rust-lang.org/1.2.0/std/primitive.str.html) to handle this.

- error-handling -- [good example](https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator) of using the `?` operator... Note that in this example, there is no panic. If the function is called like:

        let x: Result<String, io::Error> = read_username_from_file();

    ...and if the initial File::open() fails, there will be no panic. Rather, `println!("x, {:?}", x);` will yield:

        x, Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })

- iteration: iter() vs into_iter() interesting post: <https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html>

- lifetimes: re-read this a few times and play with it: <https://hellocode.dev/rust-lifetimes>

- closures: "Unlike functions, closures can capture values from the scope in which they’re defined."

    - a _fantastic_ chapter on closures: <https://doc.rust-lang.org/stable/book/ch13-01-closures.html#refactoring-using-functions>

    - this was helpful... "Note that this let statement means expensive_closure contains the definition of an anonymous function, not the resulting value of calling the anonymous function."

- iterators...

    - "The iter method produces an iterator over immutable references. If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter. Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.""

    - chapter14 has shows how simply implenting the Iterator trait by defining a next method -- automatically provides access to all the other Iterator methods (zip, map, filter, sum, maybe-more)

- smart pointers... (chapter 15)

    - "...A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data. The most common kind of pointer in Rust is a reference..."

    - "...Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities..."

- when to use usize/isize vs i32/i64

    from <https://users.rust-lang.org/t/i32-vs-isize-u32-vs-usize/22657>...

    Use usize and isize when it’s related to memory size – the size of an object, or indexing a vector, for instance. It will be a 32-bit number on 32-bit platforms, as that’s the limit of memory they can address, and likewise for 64-bit.

    Use u32 and i32 when you just want numbers.

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


Next...

<https://doc.rust-lang.org/stable/book/ch16-01-threads.html#waiting-for-all-threads-to-finish-using-join-handles>

---

