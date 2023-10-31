# rust-book-code

After
reading [Modules](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html#managing-growing-projects-with-packages-crates-and-modules)
and [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), use `mod` and `workspace` to
manage the Rust book code for future review.

**Book:**

https://doc.rust-lang.org/stable/book/

**中文翻译：**

* https://github.com/KaiserY/trpl-zh-cn (推荐)

* https://rustwiki.org/zh-CN/book/

## Usage

rust-book-code is the cargo workspace root directory, `rust_book_code` is the book code to start.

```cmd
cargo run --bin rust_book_code
```

If you want to use the code in Chapter 10, comment all chx() except ch10() in main function of rust_book_code
package.
