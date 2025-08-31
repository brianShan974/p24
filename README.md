# p24

My implementation of a 24 puzzle solver in rust.
Currently, this solver stops when it finds the first solution,
so it only finds 1 solution.

## Installation

You can either clone the repo and run

```
cargo install --path .
```

inside the project directory, or run

```
cargo install p24
```

to install from [crates.io](https://crates.io/).

## Usage

In order to use this solver,
please run

```
p24 <I1> <I2> <I3> <I4>
```

in the terminal,
where `<I1>`, `<I2>`, `<I3>` and `<I4>` should be integers.
This crate now uses `i64` as the integer type,
so please make sure that the inputs are valid `i64`s.
The reason why this crate uses `i64` instead of `i128` is to avoid overflow when calculating the sum/product/difference/quotient between `i64`s,
because all calculations are in fact done with `i128`s.
