# zhoo

...

## overview

- statically-typed language
- `hindley milner` type inference
- high performance
- `cranelift` and `llvm` backend (dev and release)
- user-friendly report messages via `ariadne`
- small binaries
- metaprogramming
- fast compilation time
- turing complete

## fresh syntax

zhoo's syntax is innovative, compact, elegant and cames with nice features. keywords are clear and have only one role. no doubt about the choice of implementation. one-liners will appreciate arrow loops (i.e. `while x < 3 -> x += 1;`), fans of ternary operations will be served with `when` expression (i.e `when true ? 1 : 2;`) but beware, type checking will throw an error if abused with nested ternary operations. `is` the keyword that means well: `x is enum::Foo` will be your best friend.

### arrow while

<p align="center">
  <img src="./misc/overview/zhoo-arrow-while.png" />
</p>

## currying

<p align="center">
  <img src="./misc/overview/zhoo-currying.png" />
</p>

## fizz buzz

<p align="center">
  <img src="./misc/overview/zhoo-fizz-buzz.png" />
</p>

## embedded unit testing

a good way to do unit tests with functionality adapted to the comfort of the programmer. logic and testing in the same file, to change files as little as possible: very useful for tdd enthusiasts.

<p align="center">
  <img src="./misc/overview/zhoo-unit-testing.png" />
</p>

## compiler phases

for the moment the proof of concept has only three phases:

```
           |--------|            |----------|            |---------|
source --> | parser | -- ast --> | analyzer | -- ast --> | codegen | --> exe
           |--------|            |----------|            |---------|
```

## start

[Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) must be installed on your machine before.

### cli

| run       | description              | cmd                                                  | status |
|:----------|:-------------------------|:-----------------------------------------------------|:-------|
| `compile` | compile with `cranelift` | `cargo run -- compile --input <path>`                | ok     |
| `compile` | compile with `llvm`      | `cargo run -- compile --input <path> --backend llvm` | ko     |
| `run`     | run the program          | `cargo run -- run                                    | ok     |

## contribution

contributions are welcome. know that there are no small contributions, so don't hesitate. i look forward to working with you.

if you have any questions, feel free to join the `zhoo` galaxy [discord](https://discord.gg/5dBTWgvb).

## license

[MIT](./LICENSE)
