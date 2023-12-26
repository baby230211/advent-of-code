# Advent of Code 2023

My solutions for [2023's Advent of Code](https://adventofcode.com/2023).

Daily solutions' code in in the [`src/bin`](src/bin) directory.

## How to Run

To run a single daily solution use:

```bash
cargo run --bin day_01_part01 inputs/01/sample.txt
cargo run --bin day_01_part01 inputs/01/input.txt
```

### 不能在相同作用域中同時存在可變和不可變引用的規則

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0] // we get immutable reference here;

v.push(6) // we use mutable reference here;
```

### 遍歷可變vector的每一個元素的可變引用

```rust
let mut v = vec![100, 32, 57];

for i in &mut v {
    *i += 50
}
```
