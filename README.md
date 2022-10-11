# zhoo

...

## overview

- statically-typed language
- `hindley milner` type inference
- high performance
- `cranelift` backend
- user-friendly report messages
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


## contribution

contributions are welcome. know that there are no small contributions, so don't hesitate. i look forward to working with you.

if you have any questions, feel free to join the `zhoo` galaxy [discord](https://discord.gg/5dBTWgvb).

## license

[MIT](./LICENSE)

