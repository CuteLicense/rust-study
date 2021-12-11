### Rust与JS或者Java的不同之处

- let

> JS 中 let 声明了变量，Rust 中 let 也用于申明变量，  
> 但是，`这个变量为不可变对象 immutable`，如果要修改变量值，需要显示的使用 `mut` 关键字进行申明

```rust
// let name = "foo"；错误！
let mut name = "foo";
name = "bar";
```

- Array

> JS 中的数组长度为可变，但是 Rust 与 JAVA 一样，`一旦定义数组长度便不可变`，且只能存储相同类型的数据，如需变化使用集合 `Vector`